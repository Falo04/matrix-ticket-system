//! This module contains the implementations for the account-related endpoints.
use super::schema::SimpleAccount;
use crate::models::account::Account;

impl From<Account> for SimpleAccount {
    fn from(value: Account) -> Self {
        Self {
            uuid: value.uuid,
            display_name: value.display_name,
            email: value.email,
            matrix_id: value.matrix_id,
        }
    }
}
