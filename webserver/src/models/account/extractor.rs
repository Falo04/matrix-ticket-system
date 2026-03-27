//! Extractor for the account model.
use galvyn::core::Module;
use galvyn::core::re_exports::axum::extract::FromRequestParts;
use galvyn::core::re_exports::axum::http::request::Parts;
use galvyn::core::session::Session;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::rorm::Database;
use uuid::Uuid;

use crate::models::account::Account;
use crate::models::account::AccountUuid;
use crate::models::account::SESSION_KEY;

impl<S> FromRequestParts<S> for Account
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    /// Parses an HTTP request part to authenticate a user.
    ///
    /// This function takes a mutable `Parts` struct containing HTTP request parts
    /// and attempts to decode a JWT token from the Authorization header.
    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        if let Some(CachedAccount(account)) = parts.extensions.get() {
            return Ok(account.clone());
        }

        let session = parts
            .extensions
            .get::<Session>()
            .ok_or(ApiError::server_error("Can't extract session."))?;

        let account_uuid = session
            .get::<Uuid>(SESSION_KEY)
            .await?
            .ok_or(ApiError::unauthorized("Missing account uuid in session"))?;

        let Some(account) =
            Account::get_by_uuid(Database::global(), &AccountUuid(account_uuid)).await?
        else {
            session.remove_value(SESSION_KEY).await?;
            session.save().await?;
            return Err(ApiError::unauthorized("Unknown account uuid in session"));
        };

        parts.extensions.insert(CachedAccount(account.clone()));

        Ok(account)
    }
}

/// A struct that represents a cached version of an `Account`.
#[derive(Clone)]
struct CachedAccount(Account);
