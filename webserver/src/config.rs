//! Configuration based on environment variables

use std::path::PathBuf;
use std::sync::LazyLock;

use galvyn::core::stuff::env::EnvError;
use galvyn::core::stuff::env::EnvVar;
use galvyn::rorm::DatabaseDriver;
use openidconnect::ClientId;
use openidconnect::ClientSecret;
use openidconnect::IssuerUrl;
use openidconnect::RedirectUrl;
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
        OIDC_DISCOVER_URL.load(),
        OIDC_CLIENT_ID.load(),
        OIDC_CLIENT_SECRET.load(),
        OIDC_REDIRECT_URL.load(),
        MATRIX_USERNAME.load(),
        MATRIX_SERVER_URL.load(),
        MATRIX_USER_PASSWORD.load(),
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

/// This variable stores the URL to use when discovering OpenID Connect (OIDC) configuration.
///
/// It is required and should be set to the URL of the OIDC provider's discovery endpoint.
/// The `EnvVar::required` ensures the variable is present in the environment.
pub static OIDC_DISCOVER_URL: EnvVar<IssuerUrl> = EnvVar::required("OIDC_DISCOVER_URL");

/// Defines an environment variable for the OIDC client ID.
///
/// This variable is required and should contain the client ID for the OIDC
/// client.
pub static OIDC_CLIENT_ID: EnvVar<ClientId> = EnvVar::required("OIDC_CLIENT_ID");

/// This variable defines the client secret used for OpenID Connect (OIDC) authentication.
///
/// It's configured via an environment variable named `OIDC_CLIENT_SECRECT`.
/// The `EnvVar::required` ensures that the variable is mandatory.
pub static OIDC_CLIENT_SECRET: EnvVar<ClientSecret> = EnvVar::required("OIDC_CLIENT_SECRET");

/// Defines an environment variable named `OIDC_REDIRECT_URL`.
///
/// This variable is required and specifies the URL to which the oidc client should return
pub static OIDC_REDIRECT_URL: EnvVar<RedirectUrl> = EnvVar::required("OIDC_REDIRECT_URL");

/// Matrix user name
pub static MATRIX_USERNAME: EnvVar = EnvVar::required("MATRIX_USERNAME");

/// Matrix user password
pub static MATRIX_USER_PASSWORD: EnvVar = EnvVar::required("MATRIX_USER_PASSWORD");

/// Matrix homeserver url
pub static MATRIX_SERVER_URL: EnvVar = EnvVar::required("MATRIX_SERVER_URL");

/// Matrix store path
pub static MATRIX_STORE_PATH: EnvVar<PathBuf> =
    EnvVar::optional("MATRIX_STORE_PATH", || PathBuf::from("/tmp/data"));
