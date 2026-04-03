//! Endpoints and schema for the frontend are defined within this module
pub mod account;
pub mod oidc;
mod tickets;

use galvyn::core::GalvynRouter;

use crate::http::middlewares::auth_required::AuthRequiredLayer;

/// Initialize the frontend routes
pub fn initialize_routes() -> GalvynRouter {
    let without_auth = GalvynRouter::new().nest("/oidc", oidc::initialize());

    let with_auth = GalvynRouter::new()
        .nest("/account", account::initialize())
        .nest("/tickets", tickets::initialize());

    without_auth.merge(with_auth.wrap(AuthRequiredLayer))
}
