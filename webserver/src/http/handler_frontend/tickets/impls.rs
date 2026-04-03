use galvyn::core::stuff::schema::SchemaDateTime;

use crate::http::handler_frontend::account::schema::SimpleAccount;
use crate::http::handler_frontend::tickets::schema::SimpleTicket;
use crate::models::tickets::Ticket;

impl From<Ticket> for SimpleTicket {
    fn from(value: Ticket) -> Self {
        Self {
            uuid: value.uuid,
            created_by: SimpleAccount::from(value.created_by),
            assigned_to: value.assigned_to.map(SimpleAccount::from),
            timestamp: SchemaDateTime(value.created_at),
            status: value.status,
            heading: value.heading,
            body: value.body,
            response: value.response,
        }
    }
}
