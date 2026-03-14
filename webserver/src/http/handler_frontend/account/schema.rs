use galvyn::core::re_exports::schemars;
use galvyn::core::re_exports::schemars::JsonSchema;
use galvyn::core::re_exports::serde::Deserialize;
use galvyn::core::re_exports::serde::Serialize;
use galvyn::rorm::fields::types::MaxStr;

use crate::models::account::AccountUuid;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SimpleAccount {
    pub uuid: AccountUuid,
    pub email: MaxStr<255>,
    pub display_name: MaxStr<255>,
}
