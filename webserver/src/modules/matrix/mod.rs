//! Matrix module

mod client;
mod event_handleres;

use galvyn::core::InitError;
use galvyn::core::Module;
use galvyn::core::PreInitError;

use crate::modules::matrix::client::MatrixClient;

///
pub struct GlobalMatrix {
    ///
    client: MatrixClient,
}

impl GlobalMatrix {}

impl Module for GlobalMatrix {
    type Setup = ();
    type PreInit = ();

    async fn pre_init((): Self::Setup) -> Result<Self::PreInit, PreInitError> {
        Ok(())
    }

    type Dependencies = ();

    async fn init((): Self::PreInit, (): &mut Self::Dependencies) -> Result<Self, InitError> {
        let client = MatrixClient::connect().await?;
        client.spawn_async_task();
        Ok(Self { client })
    }
}
