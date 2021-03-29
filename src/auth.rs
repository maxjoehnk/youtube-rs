use std::io;

use failure::Error;
use oauth2::basic::{BasicClient, BasicTokenResponse};
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, Scope,
};

static SCOPE1: &str = "https://www.googleapis.com/auth/youtube.readonly";
#[cfg(feature = "post")]
static SCOPE2: &str = "https://www.googleapis.com/auth/youtube.force-ssl";
#[cfg(not(feature = "post"))]
static SCOPE2: &str = "";

/**
 * Prints the authorize url to stdout and waits for the authorization code from stdin
 */
pub fn stdio_login(url: String) -> String {
    println!("Open this URL in your browser:\n{}\n", url);

    let mut code = String::new();
    io::stdin().read_line(&mut code).unwrap();

    code
}

// TODO: When url crate version matches we should return the url type
pub(crate) fn get_oauth_url(client: &BasicClient) -> (String, PkceCodeVerifier) {
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    let (authorize_url, _) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(SCOPE1.to_string()))
        .add_scope(Scope::new(SCOPE2.to_string()))
        .set_pkce_challenge(pkce_code_challenge)
        .add_extra_param("access_type", "offline")
        .url();
    (authorize_url.to_string(), pkce_code_verifier)
}

pub(crate) async fn request_token(
    client: &BasicClient,
    code: String,
    verifier: PkceCodeVerifier,
) -> Result<BasicTokenResponse, Error> {
    let code = AuthorizationCode::new(code);

    let token = client
        .exchange_code(code)
        .set_pkce_verifier(verifier)
        .request_async(async_http_client)
        .await?;

    Ok(token)
}

pub(crate) async fn perform_oauth<H>(
    client: &BasicClient,
    handler: H,
) -> Result<BasicTokenResponse, Error>
where
    H: Fn(String) -> String,
{
    let (authorize_url, pkce_code_verifier) = get_oauth_url(client);

    let code = handler(authorize_url);

    request_token(client, code, pkce_code_verifier).await
}
