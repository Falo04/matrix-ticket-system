//! This module contains the HTTP handlers for the account-related endpoints.
use galvyn::core::GalvynRouter;
use galvyn::openapi::OpenapiRouterExt;

mod handler;
mod impls;
pub mod schema;

/// Initializes all account-related routes
pub fn initialize() -> GalvynRouter {
    GalvynRouter::new()
        .openapi_tag("Account")
        .handler(handler::get_me)
        .handler(handler::set_matrix_id)
}
