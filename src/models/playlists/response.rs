use serde::{Deserialize, Serialize};

use crate::models::{Response, Snippet};

pub type ListPlaylistsResponse = Response<PlaylistResource>;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistResource {
    pub kind: String,
    pub etag: String,
    pub id: String,
    pub snippet: Snippet,
    pub content_details: ContentDetails
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentDetails {
    item_count: usize
}
