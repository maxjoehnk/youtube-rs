use oauth2::basic::BasicErrorResponse;
use oauth2::{RequestTokenError, url};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum YoutubeError {
    #[error("Request Token Error {0}")]
    RequestTokenError(#[from] RequestTokenError<oauth2::reqwest::Error<reqwest::Error>, BasicErrorResponse>),
    #[error("Http Error {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("Not logged in")]
    NotLoggedIn,
    #[error("Missing refresh token")]
    MissingRefreshToken,
    #[error("Invalid Url {0}")]
    InvalidUrl(#[from] url::ParseError),
    #[error("OAuth client not configured")]
    OAuthClientNotConfigured,
    #[error("No token available to persist")]
    NoTokenToPersist,
    #[error("Api Error {0}")]
    ApiError(#[from] crate::models::GoogleError),
    #[error("Json deserialization error {0}")]
    JsonDeserializeError(#[from] serde_json::Error),
    #[error("Form url encoded deserialization error {0}")]
    FormUrlEncodedDeserializeError(#[from] serde_urlencoded::de::Error),
    #[error("Unable to read token file {0}")]
    AuthFileReadError(#[source] std::io::Error),
    #[error("Unable to write token file {0}")]
    AuthFileWriteError(#[source] std::io::Error),
}

#[derive(Debug, Error)]
pub enum YoutubeDlError {
    #[error("Error executing youtube-dl command {0}")]
    CommandError(#[from] std::io::Error),
    #[error("Error parsing youtube-dl stdout {0}")]
    Utf8ParsingError(#[from] std::string::FromUtf8Error),
    #[error("youtube-dl Error {0}")]
    YoutubeDlError(String)
}
