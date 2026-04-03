use galvyn::core::re_exports::schemars;
use galvyn::core::re_exports::schemars::JsonSchema;
use galvyn::core::stuff::schema::SchemaDateTime;
use galvyn::rorm::fields::types::MaxStr;
use serde::Deserialize;
use serde::Serialize;

use crate::http::handler_frontend::account::schema::SimpleAccount;
use crate::models::tickets::TicketStatus;
use crate::models::tickets::TicketUuid;

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct SimpleTicket {
    /// The ticket's UUID.
    pub uuid: TicketUuid,
    /// The account that created the ticket.
    pub created_by: SimpleAccount,
    /// The account that the ticket is assigned to.
    pub assigned_to: Option<SimpleAccount>,
    /// The timestamp of the ticket creation.
    pub timestamp: SchemaDateTime,
    /// The status of the ticket.
    pub status: TicketStatus,
    /// The title of the ticket.
    pub heading: MaxStr<255>,
    /// The body of the ticket.
    pub body: MaxStr<1024>,
    /// The response to the ticket.
    pub response: MaxStr<1024>,
}

/// Request to set the status of a ticket.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct SetTicketStatusRequest {
    pub status: TicketStatus,
}
