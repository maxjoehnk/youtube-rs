use serde::{Deserialize, Serialize};

use crate::models::{Response, Snippet, Id};

pub type ListPlaylistsResponse = Response<PlaylistResource>;
pub type ListPlaylistItemsResponse = Response<PlaylistItemResource>;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistResource {
    pub kind: String,
    pub etag: String,
    pub id: String,
    pub snippet: Snippet
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItemResource {
    pub kind: String,
    pub etag: String,
    pub id: String,
    pub snippet: PlaylistItemSnippet,
    pub content_details: PlaylistItemContentDetails
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItemSnippet {
    pub resource_id: Id,
    #[serde(flatten)]
    pub inner: Snippet
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItemContentDetails {
    pub video_id: String,
    #[deprecated]
    pub start_at: Option<String>,
    #[deprecated]
    pub end_at: Option<String>,
    pub note: Option<String>,
    pub video_published_at: String
}
