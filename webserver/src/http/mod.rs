//! This module contains all http related code

use std::sync::OnceLock;

use galvyn::core::GalvynRouter;
use galvyn::core::SchemalessJson;
use galvyn::core::middleware::catch_unwind::CatchUnwindMiddleware;
use galvyn::get;
use galvyn::openapi::OpenAPI;
use galvyn::openapi::OpenapiRouterExt;
use galvyn::openapi::get_openapi_for_page;
use tower_http::trace::DefaultMakeSpan;
use tower_http::trace::DefaultOnResponse;
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing::instrument;

mod handler_frontend;
mod middlewares;

/// OpenAPI page for the frontend
pub struct FrontendApi;

#[get("/openapi.json")]
#[instrument]
pub async fn get_openapi() -> SchemalessJson<&'static OpenAPI> {
    SchemalessJson(galvyn::openapi::get_openapi())
}

#[get("/frontend.json")]
#[instrument]
pub async fn get_frontend_openapi() -> SchemalessJson<&'static OpenAPI> {
    static CACHE: OnceLock<OpenAPI> = OnceLock::new();
    SchemalessJson(CACHE.get_or_init(|| get_openapi_for_page(FrontendApi)))
}

/// Initialize the base router
pub fn initialize_routes() -> GalvynRouter {
    GalvynRouter::new()
        .nest(
            "/api/frontend/v1",
            handler_frontend::initialize_routes().openapi_page(FrontendApi),
        )
        .nest(
            "/docs",
            GalvynRouter::new()
                .openapi_tag("Openapi")
                .handler(get_openapi)
                .handler(get_frontend_openapi),
        )
        .wrap(CatchUnwindMiddleware::default())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO))
                // Disable automatic failure logger because any handler returning a 500 should have already logged its reason™
                .on_failure(()),
        )
}
