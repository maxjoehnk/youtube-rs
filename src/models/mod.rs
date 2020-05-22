use serde::{Serialize, Deserialize};

mod metadata;
mod search;
mod playlists;
mod snippet;

pub use self::metadata::*;
pub use self::search::*;
pub use self::playlists::*;
pub use self::snippet::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<T> {
    pub kind: String,
    pub etag: String,
    pub next_page_token: Option<String>,
    pub prev_page_token: Option<String>,
    pub region_code: Option<String>,
    pub page_info: PageInfo,
    pub items: Vec<T>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub total_results: u64,
    pub results_per_page: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum Id {
    #[serde(rename = "youtube#video")]
    VideoId {
        #[serde(rename = "videoId")]
        video_id: String
    },
    #[serde(rename = "youtube#channel")]
    ChannelId {
        #[serde(rename = "channelId")]
        channel_id: String
    },
    #[serde(rename = "youtube#playlist")]
    PlaylistId {
        #[serde(rename = "playlistId")]
        playlist_id: String
    },
}

impl Id {
    pub fn into_inner(self) -> String {
        match self {
            Id::VideoId { video_id } => video_id,
            Id::ChannelId { channel_id } => channel_id,
            Id::PlaylistId { playlist_id } => playlist_id,
        }
    }
}
