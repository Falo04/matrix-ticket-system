//! Database model for accounts.
use galvyn::rorm::Model;
use galvyn::rorm::fields::types::MaxStr;
use uuid::Uuid;

/// Represents an account in the system.
#[derive(Model, Clone, Debug)]
#[rorm(rename = "account")]
pub struct AccountModel {
    /// The user's UUID.
    #[rorm(primary_key)]
    pub uuid: Uuid,
    /// The user's matrix user id
    pub matrix_id: Option<MaxStr<1024>>,
    /// The user's display name.
    pub display_name: MaxStr<255>,
    /// The user's email
    pub email: MaxStr<255>,
    /// Identifier for the Issuer i.e., the provider
    pub issuer: MaxStr<255>,
    /// A locally unique and never reassigned identifier within the Issuer for the End-User.
    pub subject: MaxStr<255>,
}
