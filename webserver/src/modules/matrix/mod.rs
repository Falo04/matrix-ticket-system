//! Matrix module
mod on_room_invite;
mod on_room_message;

use std::path::Path;
use std::path::PathBuf;

use galvyn::core::InitError;
use galvyn::core::Module;
use galvyn::core::PreInitError;
use galvyn::core::re_exports::serde_json;
use galvyn::core::stuff::swap_lock::SwapLock;
use matrix_sdk::Client;
use matrix_sdk::authentication::matrix::MatrixSession;
use matrix_sdk::config::SyncSettings;

use crate::config::MATRIX_SERVER_URL;
use crate::config::MATRIX_STORE_PATH;
use crate::config::MATRIX_USER_PASSWORD;
use crate::config::MATRIX_USERNAME;
use crate::modules::matrix::on_room_invite::RoomInvite;
use crate::modules::matrix::on_room_message::RoomMessage;

/// The `MatrixClient` struct acts as a high-level abstraction for interacting with a Matrix server.
/// It provides the necessary components for managing communication and maintaining sync state.
pub struct MatrixClient {
    /// A `Client` instance that handles the underlying communication and interactions with the Matrix server.
    client: SwapLock<Client>,
    /// Represents the current sync token of the user's account.
    sync_token: String,
}

impl MatrixClient {
    /// Asynchronously establishes a connection to the Matrix client.
    ///
    /// This function initializes a `Client` instance configured with a homeserver URL
    /// and a SQLite store. It will then attempt to restore a previous session, if
    /// available, or initiate a new login session. The resulting `MatrixClient`
    /// instance contains the authenticated user ID and the initialized client.
    async fn connect() -> Result<Self, MatrixClientError> {
        let client = Client::builder()
            .homeserver_url(MATRIX_SERVER_URL.get().as_str())
            .sqlite_store(Self::get_matrix_store_path(), None)
            .build()
            .await
            .map_err(|err| MatrixClientError::Build(Box::new(err)))?;

        let session_path = Self::get_session_path();

        let _user_id = if session_path.exists() {
            Self::restore_session(&client, &session_path).await?
        } else {
            Self::login(&client, &session_path).await?
        };

        client.add_event_handler(RoomInvite::on_room_invite);
        client.add_event_handler(RoomMessage::on_room_message);

        let sync_token = client
            .sync_once(SyncSettings::default())
            .await
            .map_err(|err| MatrixClientError::Sync(Box::new(err)))?
            .next_batch;

        Ok(Self {
            client: SwapLock::new(client),
            sync_token,
        })
    }

    /// Logs into the Matrix server using the provided `client` and saves the session data to the specified file path.
    async fn login(client: &Client, session_path: &Path) -> Result<String, MatrixClientError> {
        client
            .matrix_auth()
            .login_username(MATRIX_USERNAME.get(), MATRIX_USER_PASSWORD.get())
            .initial_device_display_name("matrix-ticket-system")
            .send()
            .await
            .map_err(MatrixClientError::Login)?;

        if let Some(session) = client.matrix_auth().session() {
            tokio::fs::create_dir_all(MATRIX_STORE_PATH.get())
                .await
                .map_err(MatrixClientError::SessionPersist)?;
            let json =
                serde_json::to_string(&session).map_err(MatrixClientError::SessionSerialize)?;
            tokio::fs::write(session_path, json)
                .await
                .map_err(MatrixClientError::SessionPersist)?;
        }
        let user_id = client
            .user_id()
            .map(|id| id.to_string())
            .unwrap_or_else(|| "[unkown]".to_string());
        tracing::info!(%user_id, "Logged in to matrix");
        Ok(user_id)
    }

    /// Restores a session from the specified file path and uses it to initialize the provided `client`.
    async fn restore_session(
        client: &Client,
        session_path: &Path,
    ) -> Result<String, MatrixClientError> {
        let serialized = tokio::fs::read_to_string(session_path)
            .await
            .map_err(MatrixClientError::SessionRead)?;
        let session: MatrixSession =
            serde_json::from_str(&serialized).map_err(MatrixClientError::SessionParse)?;
        client
            .restore_session(session)
            .await
            .map_err(|err| MatrixClientError::SessionRestore(Box::new(err)))?;

        let user_id = client
            .user_id()
            .map(|id| id.to_string())
            .unwrap_or_else(|| "[unkown]".to_string());
        tracing::info!(%user_id, "Restored session from matrix");
        Ok(user_id)
    }

    /// Spawns an asynchronous task to perform a Matrix client synchronization.
    ///
    /// This method creates a background task using `tokio::spawn` that attempts to sync
    /// with the Matrix server using the client's `sync` method. If the synchronization fails,
    /// an error is logged using the `tracing` crate.
    fn spawn_async_task(&self) -> tokio::task::JoinHandle<()> {
        let client = self.client.get();
        let sync_token = self.sync_token.clone();

        tokio::spawn(async move {
            if let Err(err) = client.sync(SyncSettings::default().token(sync_token)).await {
                tracing::error!(%err, "Failed to sync with matrix");
            }
        })
    }

    /// Gets the matrix store path
    fn get_matrix_store_path() -> PathBuf {
        MATRIX_STORE_PATH.get().join("matrix_store")
    }

    /// Gets the matrix session path
    fn get_session_path() -> PathBuf {
        MATRIX_STORE_PATH.get().join("session.json")
    }
}

impl Module for MatrixClient {
    type Setup = ();
    type PreInit = ();

    async fn pre_init((): Self::Setup) -> Result<Self::PreInit, PreInitError> {
        Ok(())
    }

    type Dependencies = ();

    async fn init((): Self::PreInit, (): &mut Self::Dependencies) -> Result<Self, InitError> {
        let client = Self::connect().await?;
        client.spawn_async_task();
        Ok(client)
    }
}

/// The `MatrixClientError` enum represents all possible errors that
/// can occur when interacting with the Matrix client.
#[derive(Debug, thiserror::Error)]
pub enum MatrixClientError {
    /// An error occurred while building the matrix client.
    #[error("Failed to build matrix client")]
    Build(#[source] Box<dyn std::error::Error + Send + Sync>),
    /// An error occurred while serializing the session.
    #[error("Failed to serialize session")]
    SessionSerialize(#[source] serde_json::Error),
    /// An error occurred while parsing the session.
    #[error("Failed to parse session")]
    SessionParse(#[source] serde_json::Error),
    /// An error occurred while reading the session.
    #[error("Failed to read session")]
    SessionRead(#[source] std::io::Error),
    /// An error occurred while persisting the session.
    #[error("Failed to persist session")]
    SessionPersist(#[source] std::io::Error),
    /// An error occurred while restoring the session.
    #[error("Failed to restore session")]
    SessionRestore(#[source] Box<dyn std::error::Error + Send + Sync>),
    /// An error occurred while logging in to the matrix server.
    #[error("Failed to login")]
    Login(#[source] matrix_sdk::Error),
    /// An error occurred while syncing with the matrix server.
    #[error("Failed to sync with matrix")]
    Sync(#[source] Box<dyn std::error::Error + Send + Sync>),
}
