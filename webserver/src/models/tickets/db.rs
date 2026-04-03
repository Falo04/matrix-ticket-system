//! Database model for tickets.
use galvyn::rorm::Model;
use galvyn::rorm::fields::types::MaxStr;
use galvyn::rorm::prelude::ForeignModel;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::models::account::db::AccountModel;
use crate::models::tickets::TicketStatus;

/// A ticket model.
#[derive(Debug, Model)]
#[rorm(rename = "tickets")]
pub struct TicketModel {
    /// Primary key.
    #[rorm(primary_key)]
    pub uuid: Uuid,
    /// Foreign key to the account that created the ticket.
    pub created_by: ForeignModel<AccountModel>,
    /// Foreign key to the account that the ticket is assigned to.
    pub assigned_to: Option<ForeignModel<AccountModel>>,
    /// The timestamp of the ticket creation.
    pub timestamp: OffsetDateTime,
    /// The status of the ticket.
    pub status: TicketStatus,
    /// The title of the ticket.
    pub heading: MaxStr<255>,
    /// The body of the ticket.
    pub body: MaxStr<1024>,
    /// The response to the ticket.
    pub response: MaxStr<1024>,
    /// The timestamp of the ticket closure.
    pub closed_at: Option<OffsetDateTime>,
}
