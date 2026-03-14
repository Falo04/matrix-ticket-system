use galvyn::core::GalvynRouter;
use galvyn::openapi::OpenapiRouterExt;

mod handler;
pub mod schema;

pub fn initialize() -> GalvynRouter {
    GalvynRouter::new()
        .openapi_tag("Oidc")
        .handler(handler::begin_oidc_login)
        .handler(handler::finish_oidc_login)
        .handler(handler::logout)
}
