//! This module contains the schema definitions for the account-related endpoints.
use galvyn::core::re_exports::schemars;
use galvyn::core::re_exports::schemars::JsonSchema;
use galvyn::core::re_exports::serde::Deserialize;
use galvyn::core::re_exports::serde::Serialize;
use galvyn::rorm::fields::types::MaxStr;

use crate::models::account::AccountUuid;

/// A simple account representation.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SimpleAccount {
    /// The account's UUID.
    pub uuid: AccountUuid,
    /// The account's email address.
    pub email: MaxStr<255>,
    /// The account's display name.
    pub display_name: MaxStr<255>,
    /// The account's matrix user id.
    pub matrix_id: Option<MaxStr<1024>>,
}

/// Request to set the matrix user id of an account.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SetMatrixIdRequest {
    /// The matrix user id
    pub matrix_id: MaxStr<1024>,
}
