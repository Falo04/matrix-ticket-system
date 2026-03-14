use galvyn::core::re_exports::schemars;
use galvyn::core::re_exports::schemars::JsonSchema;
use galvyn::core::re_exports::serde::Deserialize;
use galvyn::core::re_exports::serde::Serialize;
use galvyn::core::stuff::schema::SchemaString;
use openidconnect::AuthorizationCode;
use openidconnect::CsrfToken;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FinishOidcLoginRequest {
    pub code: SchemaString<AuthorizationCode>,
    pub state: SchemaString<CsrfToken>,
}
