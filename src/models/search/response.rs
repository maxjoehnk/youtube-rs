use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::Response;

pub type SearchResponse = Response<SearchResult>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub kind: String,
    pub etag: String,
    pub id: Id,
    pub snippet: Snippet,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Snippet {
    #[serde(rename = "publishedAt")]
    pub published_at: String,
    #[serde(rename = "channelId")]
    pub channel_id: String,
    pub title: String,
    pub description: String,
    pub thumbnails: HashMap<String, Thumbnail>,
    #[serde(rename = "channelTitle")]
    pub channel_title: String,
    #[serde(rename = "liveBroadcastContent")]
    pub live_broadcast_content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Thumbnail {
    pub url: String,
    pub width: u64,
    pub height: u64,
}
