//! Webserver

#![deny(clippy::unwrap_used, clippy::expect_used, unsafe_code)]
#![warn(
    missing_docs,
    clippy::missing_docs_in_private_items,
    clippy::todo,
    clippy::as_conversions
)]

use std::error::Error;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;

use clap::Parser;
use galvyn::Galvyn;
use galvyn::GalvynSetup;
use galvyn::contrib::settings::ApplicationSettingsExt;
use galvyn::contrib::settings::SettingsStore;
use galvyn::core::DatabaseSetup;
use galvyn::core::re_exports::rorm;
use galvyn::rorm::Database;
use galvyn::rorm::DatabaseConfiguration;
use galvyn::rorm::config::DatabaseConfig;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::cli::Cli;
use crate::cli::Command;
use crate::config::DB;

pub mod cli;
pub mod config;
pub mod http;
pub mod models;
pub mod modules;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Err(errors) = config::load_env() {
        for error in errors {
            eprintln!("{error}");
        }
        return Err("Failed to load configuration".into());
    }

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("INFO")))
        .with(tracing_forest::ForestLayer::default().with_filter(LevelFilter::DEBUG))
        .init();

    galvyn::panic_hook::set_panic_hook();

    let cli = Cli::parse();

    match cli.command {
        Command::Start => start().await?,
        #[cfg(debug_assertions)]
        Command::MakeMigrations { migrations_dir } => make_migrations(migrations_dir).await?,
        Command::Migrate { migrations_dir } => migrate(migrations_dir).await?,
    }

    Ok(())
}

async fn start() -> Result<(), Box<dyn Error>> {
    #[expect(clippy::unit_arg)]
    Galvyn::builder(GalvynSetup::default())
        .register_module::<Database>(DatabaseSetup::Custom(DatabaseConfiguration::new(
            DB.clone(),
        )))
        .register_module::<SettingsStore>(Default::default())
        .register_module::<<modules::settings::Settings as ApplicationSettingsExt>::Module>(
            Default::default(),
        )
        .init_modules()
        .await?
        .add_routes(http::initialize_routes())
        .start(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8080))
        .await?;

    Ok(())
}

#[cfg(debug_assertions)]
async fn make_migrations(migration_dir: String) -> Result<(), Box<dyn Error>> {
    use std::io::Write;

    /// Temporary file to store models in
    const MODELS: &str = "/tmp/.models.json";

    let mut file = std::fs::File::create(MODELS)?;
    rorm::write_models(&mut file)?;
    file.flush()?;

    rorm::cli::make_migrations::run_make_migrations(
        rorm::cli::make_migrations::MakeMigrationsOptions {
            models_file: MODELS.to_string(),
            migration_dir,
            name: None,
            non_interactive: false,
            warnings_disabled: false,
        },
    )?;

    std::fs::remove_file(MODELS)?;
    Ok(())
}

async fn migrate(migration_dir: String) -> Result<(), Box<dyn Error>> {
    rorm::cli::migrate::run_migrate_custom(
        DatabaseConfig {
            driver: DB.clone(),
            last_migration_table_name: None,
        },
        migration_dir,
        false,
        None,
    )
    .await?;
    Ok(())
}
