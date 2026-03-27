//! This module contains the schema definitions for the OIDC-related endpoints.
use galvyn::core::re_exports::schemars;
use galvyn::core::re_exports::schemars::JsonSchema;
use galvyn::core::re_exports::serde::Deserialize;
use galvyn::core::re_exports::serde::Serialize;
use galvyn::core::stuff::schema::SchemaString;
use openidconnect::AuthorizationCode;
use openidconnect::CsrfToken;

/// Represents a request to complete an OIDC (OpenID Connect) login process.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FinishOidcLoginRequest {
    /// The authorization code received from the OIDC provider.
    pub code: SchemaString<AuthorizationCode>,
    /// The CSRF token received from the OIDC provider.
    pub state: SchemaString<CsrfToken>,
}
