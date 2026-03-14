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
use galvyn::rorm::db::Executor;
use galvyn::rorm::fields::types::MaxStr;
use galvyn::rorm::prelude::ForeignModel;
use tracing::instrument;
use tracing::warn;
use uuid::Uuid;

use crate::models::account::db::AccountModel;

/// Domain representation of an account used across handlers and services.
#[derive(Clone, Debug)]
pub struct Account {
    pub uuid: AccountUuid,
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
#[derive(Debug, Clone, Copy, Deserialize, Serialize, JsonSchema)]
pub struct AccountUuid(Uuid);

impl AccountUuid {
    /// Returns the inner UUID value.
    pub fn get_inner(&self) -> Uuid {
        self.0
    }

    /// Creates a new `AccountUuid` from `ForeignModel<AccountModel>`
    pub fn new_from_model(value: ForeignModel<AccountModel>) -> AccountUuid {
        AccountUuid(value.0)
    }
}

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
                // TODO
            } else {
                warn!("A session with data should have an id!");
            }
        }
        Ok(())
    }
}

impl Account {
    /// Looks up an account linked to the given OIDC issuer and subject.
    #[instrument(name = "Account::query_after_oidc", skip(exe))]
    pub async fn query_after_oidc(
        exe: impl Executor<'_>,
        issuer: &str,
        subject: &str,
    ) -> Result<Option<Account>, rorm::Error> {
        let account = rorm::query(exe, AccountModel)
            .condition(and![
                AccountModel.issuer.equals(issuer),
                AccountModel.subject.equals(subject),
            ])
            .optional()
            .await?;
        Ok(account.map(Self::from))
    }

    /// Fetches an account by its UUID.
    #[instrument(name = "Account::query_by_uuid", skip(exe))]
    pub async fn query_by_uuid(
        exe: impl Executor<'_>,
        account_uuid: &AccountUuid,
    ) -> Result<Option<Account>, rorm::Error> {
        let account = rorm::query(exe, AccountModel)
            .condition(AccountModel.uuid.equals(account_uuid.0))
            .optional()
            .await?;
        Ok(account.map(Self::from))
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
}

impl From<AccountModel> for Account {
    fn from(account_model: AccountModel) -> Self {
        Account {
            uuid: AccountUuid(account_model.uuid),
            email: account_model.email,
            display_name: account_model.display_name,
            issuer: account_model.issuer,
            subject: account_model.subject,
        }
    }
}
