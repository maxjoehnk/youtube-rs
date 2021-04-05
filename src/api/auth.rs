use failure::{ensure, Error, format_err};
use oauth2::basic::BasicClient;
use oauth2::{PkceCodeVerifier, TokenUrl, RedirectUrl, ClientId, AuthUrl, ClientSecret};
use tokio::fs::{read_to_string, write};

use crate::auth::{get_oauth_url, perform_oauth, request_token};
use crate::YoutubeApi;
use crate::token::AuthToken;
use crate::api::YoutubeOAuth;
use reqwest::Client;

pub static CODE_REDIRECT_URI: &str = "urn:ietf:wg:oauth:2.0:oob";

impl YoutubeApi {
    pub fn new_with_oauth<S: Into<String>>(api_key: S, client_id: String, client_secret: String, redirect_uri: Option<&str>) -> Result<Self, failure::Error> {
        let oauth_client = BasicClient::new(
            ClientId::new(client_id.clone()),
            Some(ClientSecret::new(client_secret.clone())),
            AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?,
            Some(TokenUrl::new(
                "https://www.googleapis.com/oauth2/v3/token".to_string(),
            )?),
        )
            .set_redirect_uri(RedirectUrl::new(
                redirect_uri.unwrap_or(CODE_REDIRECT_URI).to_string(),
            )?);

        let oauth = YoutubeOAuth {
            client_id,
            client_secret,
            client: oauth_client,
            token: AuthToken::new(),
        };

        Ok(YoutubeApi {
            api_key: api_key.into(),
            oauth: Some(oauth),
            client: Client::new(),
        })
    }

    /**
     * Perform an OAuth Login
     *
     * Available handlers:
     * * [auth::stdio_login](auth/fn.stdio_login.html)
     *
     * # Example
     * ```rust,no_run
     * use youtube_api::{YoutubeApi, auth::stdio_login};
     *
     * #[tokio::main]
     * async fn main() {
     *   let api = YoutubeApi::new_with_oauth("", String::new(), String::new(), None).unwrap();
     *
     *   api.login(stdio_login).await.unwrap();
     * }
     * ```
     */
    pub async fn login<H>(&self, handler: H) -> Result<(), Error>
        where
            H: Fn(String) -> String,
    {
        let oauth = self.oauth.as_ref().ok_or_else(|| format_err!("OAuth client not configured"))?;
        let token = perform_oauth(&oauth.client, handler).await?;
        oauth.token.set_token(token).await;
        Ok(())
    }

    pub fn get_oauth_url(&self) -> Result<(String, String), Error> {
        let oauth = self.oauth.as_ref().ok_or_else(|| format_err!("OAuth client not configured"))?;
        let (url, verifier) = get_oauth_url(&oauth.client);

        Ok((url, verifier.secret().clone()))
    }

    pub async fn request_token(&mut self, code: String, verifier: String) -> Result<(), Error> {
        let oauth = self.oauth.as_ref().ok_or_else(|| format_err!("OAuth client not configured"))?;
        let verifier = PkceCodeVerifier::new(verifier);

        let token = request_token(&oauth.client, code, verifier).await?;
        oauth.token.set_token(token).await;

        Ok(())
    }

    pub fn has_token(&self) -> bool {
        self.oauth.as_ref().map(|oauth| oauth.token.has_token()).unwrap_or_default()
    }

    /**
     * Stores the auth and refresh token in a `.google-auth.json` file for login without user input.
     */
    pub async fn store_token(&self) -> Result<(), Error> {
        let oauth = self.oauth.as_ref().ok_or_else(|| format_err!("OAuth client not configured"))?;
        ensure!(oauth.token.has_token(), "No token available to persist");
        let token = serde_json::to_string(&oauth.token.get_token().await?)?;
        write(".youtube-auth.json", token).await?; // TODO: configure file path
        Ok(())
    }

    /**
     * Stores the auth and refresh token from a `.google-auth.json` file for login without user input.
     */
    pub async fn load_token(&self) -> Result<(), Error> {
        let oauth = self.oauth.as_ref().ok_or_else(|| format_err!("OAuth client not configured"))?;
        let token = read_to_string(".youtube-auth.json").await?;
        let token = serde_json::from_str(&token)?;
        oauth.token.set_token(token).await;
        Ok(())
    }
}
