use galvyn::core::GalvynRouter;
use galvyn::openapi::OpenapiRouterExt;

mod handler;
mod schema;
mod impls;

pub fn initialize() -> GalvynRouter {
    GalvynRouter::new()
        .openapi_tag("Account")
        .handler(handler::get_me)
}
