use galvyn::core::Module;
use galvyn::core::re_exports::axum::extract::Path;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::stuff::api_json::ApiJson;
use galvyn::core::stuff::schema::List;
use galvyn::get;
use galvyn::post;
use galvyn::rorm::Database;

use crate::http::handler_frontend::tickets::schema::SetTicketStatusRequest;
use crate::http::handler_frontend::tickets::schema::SimpleTicket;
use crate::models::account::Account;
use crate::models::tickets::Ticket;
use crate::models::tickets::TicketUuid;

#[get("/all")]
pub async fn get_tickets(user: Account) -> ApiResult<ApiJson<List<SimpleTicket>>> {
    let mut tx = Database::global().start_transaction().await?;
    let tickets = Ticket::get_by_created_by(&mut tx, user.uuid).await?;
    tx.commit().await?;
    Ok(ApiJson(List {
        list: tickets.into_iter().map(SimpleTicket::from).collect(),
    }))
}

#[post("/update-status/{ticket_uuid}")]
pub async fn update_status(
    Path(ticket_uuid): Path<TicketUuid>,
    ApiJson(request): ApiJson<SetTicketStatusRequest>,
) -> ApiResult<()> {
    let mut tx = Database::global().start_transaction().await?;

    let mut ticket = Ticket::get_by_uuid(&mut tx, ticket_uuid)
        .await?
        .ok_or(ApiError::bad_request("Ticket not found"))?;
    ticket.update_status(&mut tx, request.status).await?;

    tx.commit().await?;
    Ok(())
}
