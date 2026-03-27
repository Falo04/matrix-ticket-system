//! Handles room messages.
use std::string::ToString;
use std::sync::LazyLock;

use galvyn::core::Module;
use galvyn::core::re_exports::rorm;
use galvyn::rorm::Database;
use galvyn::rorm::fields::types::MaxStr;
use matrix_sdk::Room;
use matrix_sdk::RoomState;
use matrix_sdk::ruma::UserId;
use matrix_sdk::ruma::events::room::message::MessageType;
use matrix_sdk::ruma::events::room::message::OriginalSyncRoomMessageEvent;
use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use regex::Regex;
use thiserror::Error;
use tracing::error;

use crate::models::account::Account;
use crate::models::tickets::CreateTicket;
use crate::models::tickets::Ticket;

/// Handles room messages.
pub struct RoomMessage;

/// Regex for parsing ticket requests.
#[allow(clippy::unwrap_used)]
static TICKET_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)!ticket\s+(.*?)\s*\+++\s*(.*?)\s*\+++\s*(.*)").unwrap());

/// Template message to send when the user requests a ticket.
static TEMPLATE_MSG: LazyLock<String> = LazyLock::new(|| {
    "🎫 **Ticket Template**\n\n\
                    Use this exact format:\n\
                    ```\n\
                    !ticket\n\
                    Your Heading Here\n\
                    +++\n\
                    Your detailed description goes here.\n\
                    It can span multiple lines.\n\
                    +++\n\
                    John Doe\n\
                    ```\n\n\
                    **Note:** Use `---` on its own line to separate the sections.\n\
                    *Limits: Heading 255 chars, Body 1024 chars.*"
        .to_string()
});

impl RoomMessage {
    /// Handle messages sent in the room.
    pub async fn on_room_message(event: OriginalSyncRoomMessageEvent, room: Room) {
        if room.state() != RoomState::Joined {
            return;
        }

        let MessageType::Text(text_content) = event.content.msgtype else {
            return;
        };
        let body = text_content.body.trim();

        if body == "!template" {
            let _ = room
                .send(RoomMessageEventContent::text_markdown(TEMPLATE_MSG.clone()))
                .await;
            return;
        }

        if body.starts_with("!ticket") {
            let response_text =
                match Self::handle_ticket_request(&text_content.body, &event.sender).await {
                    Ok(_) => "Ticket created successfully".to_string(),
                    Err(err) => err.to_user_message(),
                };

            let _ = room
                .send(RoomMessageEventContent::text_plain(response_text))
                .await;
        }
    }

    /// Handles a ticket creation request by processing the provided content and creating a new ticket.
    async fn handle_ticket_request(
        text_content: &str,
        sender: &UserId,
    ) -> Result<(), TicketHandlerErrors> {
        let mut tx = Database::global().start_transaction().await?;
        let matrix_id = sender.to_string();

        let account = Account::get_by_matrix_id(&mut tx, &matrix_id)
            .await?
            .ok_or(TicketHandlerErrors::AccountNotFound)?;

        let caps = TICKET_REGEX
            .captures(text_content)
            .ok_or(TicketHandlerErrors::ParseBody)?;

        let assigned_to = Account::get_by_display_name(&mut tx, &caps[3]).await?;

        Ticket::create(
            &mut tx,
            CreateTicket {
                created_by: account.uuid,
                heading: MaxStr::new(caps[1].trim().to_string())
                    .map_err(|_| TicketHandlerErrors::HeadingTooLong)?,
                body: MaxStr::new(caps[2].trim().to_string())
                    .map_err(|_| TicketHandlerErrors::BodyTooLong)?,
                assigned_to: assigned_to.map(|a| a.uuid),
            },
        )
        .await?;

        tx.commit().await?;
        Ok(())
    }
}

/// Errors that can occur during ticket handling.
#[derive(Debug, Error)]
pub enum TicketHandlerErrors {
    /// An error occurred while database operations.
    #[error("Database error: {0:?}")]
    Database(#[from] rorm::Error),
    /// The account could not be found.
    #[error("Account not found")]
    AccountNotFound,
    /// The body could not be parsed.
    #[error("Failed to parse body")]
    ParseBody,
    /// The heading is too long.
    #[error("Heading too long. Max length is 255 characters.")]
    HeadingTooLong,
    /// The body is too long.
    #[error("Body too long. Max length is 1024 characters.")]
    BodyTooLong,
}

impl TicketHandlerErrors {
    /// Converts the error to a user-friendly message.
    pub fn to_user_message(&self) -> String {
        match self {
            Self::AccountNotFound => "Account not found".into(),
            Self::ParseBody => {
                "Failed to parse body, please use !template to get the correct template.".into()
            }
            Self::HeadingTooLong => "Heading too long, only 255 characters allowed".into(),
            Self::BodyTooLong => "Body too long, you can only have 1024 characters".into(),
            Self::Database(_) => {
                error!(error.display = %self, error.debug = ?self, "Database error");
                "Database error please contact your admin".into()
            }
        }
    }
}
