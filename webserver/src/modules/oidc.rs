//! This module provides functionality for interacting with an OpenID Connect provider.
//!
//! It manages the login flow, exchanging authorization codes for access tokens and ID tokens.
use std::time::Duration;

use galvyn::core::re_exports::serde::Deserialize;
use galvyn::core::re_exports::serde::Serialize;
use galvyn::core::stuff::api_error::ApiError;
use galvyn::core::stuff::api_error::ApiResult;
use galvyn::core::InitError;
use galvyn::core::Module;
use galvyn::core::PreInitError;
use openidconnect::core::CoreAuthenticationFlow;
use openidconnect::core::CoreClient;
use openidconnect::core::CoreIdTokenClaims;
use openidconnect::core::CoreProviderMetadata;
use openidconnect::reqwest;
use openidconnect::AccessTokenHash;
use openidconnect::AuthorizationCode;
use openidconnect::ClientId;
use openidconnect::ClientSecret;
use openidconnect::CsrfToken;
use openidconnect::DiscoveryError;
use openidconnect::EndpointMaybeSet;
use openidconnect::EndpointNotSet;
use openidconnect::EndpointSet;
use openidconnect::HttpClientError;
use openidconnect::IssuerUrl;
use openidconnect::Nonce;
use openidconnect::OAuth2TokenResponse;
use openidconnect::PkceCodeChallenge;
use openidconnect::PkceCodeVerifier;
use openidconnect::RedirectUrl;
use openidconnect::RequestTokenError;
use openidconnect::Scope;
use openidconnect::TokenResponse;
use tracing::error;
use tracing::warn;
use url::Url;

use crate::config::OIDC_CLIENT_ID;
use crate::config::OIDC_CLIENT_SECRET;
use crate::config::OIDC_DISCOVER_URL;
use crate::config::OIDC_REDIRECT_URL;

/// Represents an OpenID Connect client.
///
/// This struct encapsulates the necessary components for interacting with
/// an OpenID Connect provider. It includes a `reqwest::Client` for
/// making HTTP requests and an `OidcClient` for handling OIDC-specific
/// logic.
pub struct OpenIdConnect {
    /// A `reqwest::Client` used for making HTTP requests.
    http_client: reqwest::Client,
    /// An `OidcClient` instance for handling OpenID Connect details.
    oidc_client: OidcClient,
}

type OidcClient = CoreClient<
    EndpointSet, // Auth URL
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet, // Token URL
    EndpointMaybeSet,
>;

/// Represents the state of an OpenID Connect session.
///
/// This struct contains the necessary information to maintain a secure and authenticated session.
/// It includes the CSRF token, PKCE code verifier, nonce, and side query parameter.
#[derive(Debug, Deserialize, Serialize)]
pub struct OidcSessionState {
    /// The CSRF token for cross-site request forgery protection.
    pub csrf_token: CsrfToken,
    /// The PKCE code verifier used for Proof Key for Code Exchange.
    pub pkce_code_verifier: PkceCodeVerifier,
    /// A unique, non-reusable value used to prevent replay attacks.
    pub nonce: Nonce,
}

/// Represents the state of an OpenID Connect authorization code request.
///
/// This struct holds the authorization code and a CSRF token, which are
/// necessary for securely handling the request.
#[derive(Debug, Serialize, Deserialize)]
pub struct OidcRequestState {
    /// An `AuthorizationCode` representing the authorization code obtained
    /// from the authorization server.
    pub code: AuthorizationCode,
    /// A `CsrfToken` used to prevent Cross-Site Request Forgery (CSRF) attacks.
    pub state: CsrfToken,
}

impl OpenIdConnect {
    /// Initiates the OIDC authorization flow for a given side.
    ///
    /// This function constructs the authorization URL and returns the necessary
    /// session state information.
    pub fn begin_login(&self) -> anyhow::Result<(Url, OidcSessionState)> {
        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

        let request = self
            .oidc_client
            .authorize_url(
                CoreAuthenticationFlow::AuthorizationCode,
                CsrfToken::new_random,
                Nonce::new_random,
            )
            .set_pkce_challenge(pkce_code_challenge)
            .add_scope(Scope::new("profile".to_string()))
            .add_scope(Scope::new("email".to_string()));

        let (auth_url, csrf_token, nonce) = request.url();

        Ok((
            auth_url,
            OidcSessionState {
                csrf_token,
                pkce_code_verifier,
                nonce,
            },
        ))
    }

    /// Finishes the login process by exchanging the authorization code for an ID token,
    /// verifying the ID token, and optionally validating the access token signature
    pub async fn finish_login(
        &self,
        session: OidcSessionState,
        request: OidcRequestState,
    ) -> ApiResult<CoreIdTokenClaims> {
        if request.state != session.csrf_token {
            return Err(ApiError::unauthorized("Secret state is invalid"));
        }

        // Exchange the authorization code with a token.
        let token_response = self
            .oidc_client
            .exchange_code(request.code)
            .set_pkce_verifier(session.pkce_code_verifier)
            .request_async(&self.http_client)
            .await
            .map_err(|error| match error {
                RequestTokenError::ServerResponse(_) => {
                    ApiError::server_error("server response invalid")
                }
                _ => ApiError::bad_request("bad request"),
            })?;

        let id_token = token_response.id_token().ok_or(ApiError::server_error(
            "Oidc provider did not provider an id token.",
        ))?;
        let id_token_verifier = self.oidc_client.id_token_verifier();
        let claims = id_token
            .claims(&id_token_verifier, &session.nonce)
            .map_err(|error| {
                ApiError::unauthorized("Failed to verify id token").with_source(error)
            })?;

        if let Some(expected_access_token_hash) = claims.access_token_hash() {
            let actual_access_token_hash = AccessTokenHash::from_token(
                token_response.access_token(),
                id_token.signing_alg().map_err(ApiError::map_server_error(
                    "Failed to retrieve signing algorithm",
                ))?,
                id_token
                    .signing_key(&id_token_verifier)
                    .map_err(ApiError::map_server_error("Failed to retrieve signing key"))?,
            )
                .map_err(ApiError::map_server_error(
                    "Failed to recreate access token signature",
                ))?;
            if actual_access_token_hash != *expected_access_token_hash {
                return Err(ApiError::unauthorized("Invalid access token"));
            }
        }

        Ok(claims.clone())
    }
}

impl Module for OpenIdConnect {
    type Setup = ();
    type PreInit = Self;

    async fn pre_init(_setup: Self::Setup) -> Result<Self::PreInit, PreInitError> {
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .expect("Should create a http client");

        let oidc_client = OidcConfig::from_env()
            .discover_retry::<3>(&http_client)
            .await?;

        Ok(Self {
            http_client,
            oidc_client,
        })
    }

    type Dependencies = ();

    async fn init(
        pre_init: Self::PreInit,
        _dependencies: &mut Self::Dependencies,
    ) -> Result<Self, InitError> {
        Ok(pre_init)
    }
}

/// Represents the configuration for an OpenID Connect (OIDC) flow.
///
/// This struct holds the necessary parameters for establishing a connection with an OIDC provider.
struct OidcConfig {
    /// The URL of the OIDC issuer.
    url: IssuerUrl,
    /// The client ID for the application.
    client_id: ClientId,
    /// The client secret for the application.
    client_secret: ClientSecret,
    /// The URL where the OIDC provider will redirect the user after authentication.
    redirect_url: RedirectUrl,
}

impl OidcConfig {
    /// Creates a new `OidcConfig` instance.
    ///
    /// This function initializes the configuration with the provided values.
    fn from_env() -> Self {
        OidcConfig {
            url: OIDC_DISCOVER_URL.clone(),
            client_id: OIDC_CLIENT_ID.clone(),
            client_secret: OIDC_CLIENT_SECRET.clone(),
            redirect_url: OIDC_REDIRECT_URL.clone(),
        }
    }

    /// Attempts to discover an OIDC client with retries.
    ///
    /// This function retries the `discover` function `N` times, handling potential
    /// `reqwest::Error`s and timeouts.
    async fn discover_retry<const N: usize>(
        &self,
        http_client: &reqwest::Client,
    ) -> Result<OidcClient, DiscoveryError<HttpClientError<reqwest::Error>>> {
        let mut result = Err(DiscoveryError::Other(String::new()));
        for _ in 0..N {
            result = self.discover(http_client).await;
            if let Err(DiscoveryError::Request(HttpClientError::Reqwest(error))) = &result {
                if error.is_timeout() {
                    warn!("Timed out fetching oidc discovery, trying again...");
                    continue;
                }
            }
            return result;
        }
        error!("Timed out fetching oidc discovery");
        result
    }

    /// Discovers the OIDC client configuration.
    ///
    /// This function uses the provided HTTP client to discover the OIDC client
    /// configuration based on the issuer's metadata.
    async fn discover(
        &self,
        http_client: &reqwest::Client,
    ) -> Result<OidcClient, DiscoveryError<HttpClientError<reqwest::Error>>> {
        let oidc_client = CoreClient::from_provider_metadata(
            CoreProviderMetadata::discover_async(self.url.clone(), http_client).await?,
            self.client_id.clone(),
            Some(self.client_secret.clone()),
        )
            .set_redirect_uri(self.redirect_url.clone());

        // Check the token url to be set
        let token_uri = oidc_client
            .token_uri()
            .ok_or_else(|| DiscoveryError::Other("Issuer did not provide a token url".to_string()))?
            .clone();
        let oidc_client = oidc_client.set_token_uri(token_uri);

        Ok(oidc_client)
    }
}
