//! Configuration based on environment variables

use std::sync::LazyLock;

use galvyn::core::stuff::env::EnvError;
use galvyn::core::stuff::env::EnvVar;
use galvyn::rorm::DatabaseDriver;
use url::Url;

/// Load all environment variables declared in this module
///
/// Called at the beginning of `main` to gather and report all env errors at once.
pub fn load_env() -> Result<(), Vec<&'static EnvError>> {
    let mut errors = Vec::new();

    for result in [
        ORIGIN.load(),
        POSTGRES_DB.load(),
        POSTGRES_USER.load(),
        POSTGRES_PASSWORD.load(),
    ] {
        errors.extend(result.err());
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// The url this server is reachable under
///
/// # Used for
/// - generating links which should point back to webserver
pub static ORIGIN: EnvVar<Url> = EnvVar::required("ORIGIN");

/// The database name
pub static POSTGRES_DB: EnvVar = EnvVar::required("POSTGRES_DB");

/// The user to use for the database connection
pub static POSTGRES_USER: EnvVar = EnvVar::optional("POSTGRES_USER", || "postgres".to_string());

/// Password for the user
pub static POSTGRES_PASSWORD: EnvVar = EnvVar::optional("POSTGRES_PASSWORD", || "".to_string());

/// Bundle of all database variables combined in `rorm`'s format
pub static DB: LazyLock<DatabaseDriver> = LazyLock::new(|| DatabaseDriver::Postgres {
    name: POSTGRES_DB.clone(),
    host: "postgres".to_string(),
    port: 5432,
    user: POSTGRES_USER.clone(),
    password: POSTGRES_PASSWORD.clone(),
});
