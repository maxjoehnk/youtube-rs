pub use api::YoutubeApi;
pub use youtube_dl::YoutubeDl;
pub use self::error::YoutubeError;
pub(crate) type Result<T> = std::result::Result<T, YoutubeError>;

mod api;
pub mod auth;
pub mod models;
mod token;
mod youtube_dl;
mod error;
