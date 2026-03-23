use matrix_sdk::Client;
use matrix_sdk::Room;
use matrix_sdk::RoomState;
use matrix_sdk::ruma::events::room::member::StrippedRoomMemberEvent;
use matrix_sdk::ruma::events::room::message::MessageType;
use matrix_sdk::ruma::events::room::message::OriginalSyncRoomMessageEvent;
use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;

pub struct EventHandlers;

impl EventHandlers {
    /// Auto-accept room invites addressed to us.
    pub async fn on_room_invite(event: StrippedRoomMemberEvent, client: Client, room: Room) {
        let Some(user_id) = client.user_id() else {
            return;
        };

        if event.state_key != user_id {
            return;
        }

        let room_id = room.room_id().to_owned();
        tracing::info!(%room_id, "Got invite to room");

        tokio::spawn(async move {
            match room.join().await {
                Ok(_) => tracing::info!(%room_id, "Joined room"),
                Err(err) => tracing::error!(%err, %room_id, "Failed to join room"),
            }
        });
    }

    /// Handle messages sent in the room.
    pub async fn on_room_message(event: OriginalSyncRoomMessageEvent, room: Room) {
        if room.state() != RoomState::Joined {
            return;
        }

        let MessageType::Text(text_content) = event.content.msgtype else {
            return;
        };
        tracing::info!(%text_content.body, "Got message");

        if text_content.body.contains("!ticket") {
            let content = RoomMessageEventContent::text_plain("Received ticket");
            tracing::info!("Sending message");
            match room.send(content).await {
                Ok(_) => tracing::info!("Message sent"),
                Err(err) => tracing::error!(%err, "Failed to send message"),
            };
        }
    }
}
