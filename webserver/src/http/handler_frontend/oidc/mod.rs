//! This module contains the HTTP handlers for the OIDC-related endpoints.
use galvyn::core::GalvynRouter;
use galvyn::openapi::OpenapiRouterExt;

mod handler;
pub mod schema;

/// Initializes all OIDC-related routes
pub fn initialize() -> GalvynRouter {
    GalvynRouter::new()
        .openapi_tag("Oidc")
        .handler(handler::begin_oidc_login)
        .handler(handler::finish_oidc_login)
        .handler(handler::logout)
}
