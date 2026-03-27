//! This module contains the HTTP handlers for the account-related endpoints.
use galvyn::core::Module;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::get;
use galvyn::post;
use galvyn::rorm::Database;

use crate::http::handler_frontend::account::schema::SetMatrixIdRequest;
use crate::http::handler_frontend::account::schema::SimpleAccount;
use crate::models::account::Account;

/// This function handles requests to the "/me" endpoint.
#[get("/me")]
pub async fn get_me(user: Account) -> ApiResult<ApiJson<SimpleAccount>> {
    Ok(ApiJson(SimpleAccount::from(user)))
}

#[post("/me/set-matrix-id")]
pub async fn set_matrix_id(
    user: Account,
    ApiJson(request): ApiJson<SetMatrixIdRequest>,
) -> ApiResult<()> {
    let mut tx = Database::global().start_transaction().await?;
    user.set_matrix_id(&mut tx, request.matrix_id).await?;
    tx.commit().await?;
    Ok(())
}
