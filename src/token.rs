use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::{Duration, Instant};

use failure::{format_err, Error};
use log::debug;
use oauth2::basic::{BasicClient, BasicTokenResponse};
use oauth2::reqwest::async_http_client;
use oauth2::TokenResponse;

#[derive(Debug, Clone)]
pub(crate) struct AuthToken {
    token: Arc<Mutex<Option<BasicTokenResponse>>>,
    has_token: Arc<AtomicBool>,
    expired_at: Arc<Mutex<Instant>>,
}

impl AuthToken {
    pub(crate) fn new() -> AuthToken {
        AuthToken {
            token: Arc::new(Mutex::new(None)),
            has_token: Arc::new(AtomicBool::new(false)),
            expired_at: Arc::new(Mutex::new(Instant::now())),
        }
    }

    pub(crate) async fn set_token(&self, new_token: BasicTokenResponse) {
        let mut token = self.token.lock().await;
        let mut expired_at = self.expired_at.lock().await;
        *expired_at = Instant::now()
            + new_token
                .expires_in()
                .unwrap_or_else(|| Duration::new(0, 0));
        *token = Some(new_token);
        self.has_token.store(true, Ordering::Relaxed);
    }

    pub(crate) async fn get_token(&self) -> Result<BasicTokenResponse, Error> {
        Ok(self
            .token
            .lock()
            .await
            .as_ref()
            .ok_or_else(|| format_err!("Not logged in"))?
            .clone())
    }

    pub(crate) fn has_token(&self) -> bool {
        self.has_token.load(Ordering::Relaxed)
    }

    pub(crate) async fn refresh(&self, client: &BasicClient) -> Result<(), Error> {
        debug!("refreshing access token");
        let token = {
            let token = self.token.lock().await;
            let refresh_token = token
                .as_ref()
                .ok_or_else(|| format_err!("Not logged in"))?
                .refresh_token()
                .ok_or_else(|| format_err!("No refresh token"))?;

            client
                .exchange_refresh_token(refresh_token)
                .request_async(async_http_client)
                .await
        }?;

        self.set_access_token(token).await;

        Ok(())
    }

    async fn set_access_token(&self, new_token: BasicTokenResponse) {
        let mut token = self.token.lock().await;
        if let Some(token) = token.as_mut() {
            token.set_access_token(new_token.access_token().clone());
            let mut expired_at = self.expired_at.lock().await;
            *expired_at = Instant::now()
                + new_token
                    .expires_in()
                    .unwrap_or_else(|| Duration::new(0, 0));
            self.has_token.store(true, Ordering::Relaxed);
        }
    }

    pub(crate) async fn get_auth_header(&self) -> Result<String, Error> {
        let token = self.token.lock().await;
        let token = token
            .as_ref()
            .ok_or_else(|| format_err!("Not logged in"))?
            .access_token()
            .secret();
        Ok(token.clone())
    }

    pub(crate) async fn requires_new_token(&self) -> bool {
        let has_token = self.has_token.load(Ordering::Relaxed);
        if !has_token {
            true
        } else {
            let expired = self.expired_at.lock().await;
            Instant::now().ge(&expired)
        }
    }
}
