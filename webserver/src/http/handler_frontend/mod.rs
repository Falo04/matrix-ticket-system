//! Endpoints and schema for the frontend are defined within this module

use galvyn::core::GalvynRouter;

/// Initialize the frontend routes
pub fn initialize_routes() -> GalvynRouter {
    GalvynRouter::new()
}
