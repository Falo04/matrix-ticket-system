//! Account domain model and session-backed authentication extractor.
pub(in crate::models) mod db;
mod extractor;

use galvyn::core::re_exports::rorm;
use galvyn::core::re_exports::schemars;
use galvyn::core::re_exports::schemars::JsonSchema;
use galvyn::core::re_exports::serde::Deserialize;
use galvyn::core::re_exports::serde::Serialize;
use galvyn::core::session;
use galvyn::core::session::Session;
use galvyn::rorm::and;
use galvyn::rorm::conditions::Condition;
use galvyn::rorm::db::Executor;
use galvyn::rorm::fields::types::MaxStr;
use tracing::instrument;
use tracing::warn;
use utility_macros::BusinessModelUuid;
use uuid::Uuid;

use crate::models::account::db::AccountModel;
use crate::utils::bm_uuid::BusinessModelUuid;

/// Domain representation of an account used across handlers and services.
#[derive(Clone, Debug)]
pub struct Account {
    /// The user's UUID.
    pub uuid: AccountUuid,
    /// The user's matrix user id
    pub matrix_id: Option<MaxStr<1024>>,
    /// The user's display name.
    pub display_name: MaxStr<255>,
    /// The user's email
    pub email: MaxStr<255>,
    /// Identifier for the Issuer i.e. the provider
    pub issuer: MaxStr<255>,
    /// A locally unique and never reassigned identifier within the Issuer for the End-User.
    pub subject: MaxStr<255>,
}

/// Wrapper type to give stronger typing to account identifiers.
#[derive(Debug, Clone, Copy, BusinessModelUuid, Deserialize, Serialize, JsonSchema)]
#[bm_uuid(model = "AccountModel")]
pub struct AccountUuid(Uuid);

/// Data for inserting a new account.
#[derive(Clone, Debug)]
pub struct InsertAccount {
    /// The user's display name.
    pub display_name: MaxStr<255>,
    /// The user's email
    pub email: MaxStr<255>,
    /// Identifier for the Issuer i.e. the provider
    pub issuer: MaxStr<255>,
    /// A locally unique and never reassigned identifier within the Issuer for the End-User.
    pub subject: MaxStr<255>,
}

/// Session key for storing the account UUID.
const SESSION_KEY: &str = "current_account";

impl Account {
    /// Marks this account as logged in by storing its UUID in the session.
    #[instrument(name = "Account::set_logged_in", skip(self))]
    pub async fn set_logged_in(&self, session: &Session) -> Result<(), session::Error> {
        session.insert(SESSION_KEY, self.uuid).await?;
        Ok(())
    }

    /// Clears the login state by removing the account UUID from the session.
    #[instrument(name = "Account::unset_logged_in")]
    pub async fn unset_logged_in(session: Session) -> Result<(), session::Error> {
        if let Some(_account_uuid) = session.remove::<Uuid>(SESSION_KEY).await? {
            if let Some(_session_id) = session.id() {
                // TODO send websocket notification
            } else {
                warn!("A session with data should have an id!");
            }
        }
        Ok(())
    }
}

impl Account {
    /// Looks up an account linked to the given OIDC issuer and subject.
    #[instrument(name = "Account::get_by_oidc", skip(exe))]
    pub async fn get_by_oidc(
        exe: impl Executor<'_>,
        issuer: &str,
        subject: &str,
    ) -> Result<Option<Account>, rorm::Error> {
        Self::get_by_condition(
            exe,
            and![
                AccountModel.issuer.equals(issuer),
                AccountModel.subject.equals(subject),
            ],
        )
        .await
    }

    /// Fetches an account by its UUID.
    #[instrument(name = "Account::get_by_uuid", skip(exe))]
    pub async fn get_by_uuid(
        exe: impl Executor<'_>,
        account_uuid: &AccountUuid,
    ) -> Result<Option<Account>, rorm::Error> {
        Self::get_by_condition(exe, AccountModel.uuid.equals(account_uuid.0)).await
    }

    /// Fetches an account by its matrix user id.
    #[instrument(name = "Account::get_by_matrix_id", skip(exe))]
    pub async fn get_by_matrix_id(
        exe: impl Executor<'_>,
        matrix_id: &str,
    ) -> Result<Option<Account>, rorm::Error> {
        Self::get_by_condition(
            exe,
            AccountModel.matrix_id.equals(Some(matrix_id.to_string())),
        )
        .await
    }

    /// Fetches an account by its display name.
    #[instrument(name = "Account::get_by_display_name", skip(exe))]
    pub async fn get_by_display_name(
        exe: impl Executor<'_>,
        display_name: &str,
    ) -> Result<Option<Account>, rorm::Error> {
        Self::get_by_condition(
            exe,
            AccountModel.display_name.equals(display_name.to_string()),
        )
        .await
    }

    /// Creates a new account record.
    #[instrument(name = "Account::create", skip(exe))]
    pub async fn create(
        exe: impl Executor<'_>,
        data: InsertAccount,
    ) -> Result<Account, rorm::Error> {
        let account_model = rorm::insert(exe, AccountModel)
            .single(&AccountModel {
                uuid: Uuid::new_v4(),
                matrix_id: None,
                email: data.email,
                display_name: data.display_name,
                issuer: data.issuer,
                subject: data.subject,
            })
            .await?;

        Ok(Account::from(account_model))
    }

    /// Updates an existing account record.
    #[instrument(name = "Account::update", skip(exe))]
    pub async fn update(
        &self,
        exe: impl Executor<'_>,
        display_name: MaxStr<255>,
    ) -> anyhow::Result<()> {
        rorm::update(exe, AccountModel)
            .set(AccountModel.display_name, display_name)
            .condition(AccountModel.uuid.equals(self.uuid.0))
            .await?;
        Ok(())
    }

    /// Sets the matrix user id for the account.
    #[instrument(name = "Account::set_matrix_id", skip(exe))]
    pub async fn set_matrix_id(
        &self,
        exe: impl Executor<'_>,
        matrix_id: MaxStr<1024>,
    ) -> anyhow::Result<()> {
        rorm::update(exe, AccountModel)
            .set(AccountModel.matrix_id, Some(matrix_id))
            .condition(AccountModel.uuid.equals(self.uuid.0))
            .await?;
        Ok(())
    }

    /// Asynchronously retrieves an objects based on a specified condition from the database.
    pub async fn get_by_condition(
        exe: impl Executor<'_>,
        condition: impl Condition<'_>,
    ) -> Result<Option<Account>, rorm::Error> {
        let account = rorm::query(exe, AccountModel)
            .condition(condition)
            .optional()
            .await?;
        Ok(account.map(Self::from))
    }
}

impl From<AccountModel> for Account {
    fn from(account_model: AccountModel) -> Self {
        Account {
            uuid: AccountUuid(account_model.uuid),
            matrix_id: account_model.matrix_id,
            email: account_model.email,
            display_name: account_model.display_name,
            issuer: account_model.issuer,
            subject: account_model.subject,
        }
    }
}
