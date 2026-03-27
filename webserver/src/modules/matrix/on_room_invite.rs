//! Handles room invites.
use matrix_sdk::Client;
use matrix_sdk::Room;
use matrix_sdk::ruma::events::room::member::StrippedRoomMemberEvent;

/// Handles room invites.
pub struct RoomInvite;

impl RoomInvite {
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
}
