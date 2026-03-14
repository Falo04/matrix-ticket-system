//! This function implements an authentication layer for Axum routes.
//!
//! It attempts to extract a user from the incoming request. If successful,
//! it passes the request to the next handler in the chain. If an error
//! occurs during user extraction, it returns the error.

use std::ops::ControlFlow;

use galvyn::core::middleware::SimpleGalvynMiddleware;
use galvyn::core::re_exports::axum::extract::FromRequestParts;
use galvyn::core::re_exports::axum::extract::Request;
use galvyn::core::re_exports::axum::response::IntoResponse;
use galvyn::core::re_exports::axum::response::Response;

use crate::models::account::Account;

#[derive(Copy, Clone, Debug)]
pub struct AuthRequiredLayer;

impl SimpleGalvynMiddleware for AuthRequiredLayer {
    async fn pre_handler(&mut self, req: Request) -> ControlFlow<Response, Request> {
        let (mut parts, body) = req.into_parts();
        match Account::from_request_parts(&mut parts, &()).await {
            Ok(_account) => ControlFlow::Continue(Request::from_parts(parts, body)),
            Err(rejection) => ControlFlow::Break(rejection.into_response()),
        }
    }
}
