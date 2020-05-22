use serde::{Deserialize, Serialize};

use crate::models::{Response, Snippet};

pub type SearchResponse = Response<SearchResult>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub kind: String,
    pub etag: String,
    pub id: Id,
    pub snippet: Snippet,
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
