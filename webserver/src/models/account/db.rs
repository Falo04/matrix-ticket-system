use galvyn::rorm::fields::types::MaxStr;
use galvyn::rorm::prelude::ForeignModel;
use galvyn::rorm::Model;
use uuid::Uuid;

/// Represents an account in the system.
#[derive(Model, Clone, Debug)]
#[rorm(rename = "account")]
pub struct AccountModel {
    #[rorm(primary_key)]
    pub uuid: Uuid,
    /// The user's display name.
    pub display_name: MaxStr<255>,
    /// The user's email
    pub email: MaxStr<255>,
    /// Identifier for the Issuer i.e. the provider
    pub issuer: MaxStr<255>,
    /// A locally unique and never reassigned identifier within the Issuer for the End-User.
    pub subject: MaxStr<255>,
}
