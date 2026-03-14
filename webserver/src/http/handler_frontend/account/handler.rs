use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::get;

use crate::http::handler_frontend::account::schema::SimpleAccount;
use crate::models::account::Account;

/// This function handles requests to the "/me" endpoint.
#[get("/me")]
pub async fn get_me(user: Account) -> ApiResult<ApiJson<SimpleAccount>> {
    Ok(ApiJson(SimpleAccount::from(user)))
}
