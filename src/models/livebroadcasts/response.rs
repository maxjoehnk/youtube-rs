use serde::{Deserialize, Serialize};

use crate::models::{Response, Snippet, Id};

pub type ListLiveBroadcastResponse = Response<LiveBroadcastResource>;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveBroadcastResource {
    pub kind: String,
    pub etag: String,
    pub id: String,
    pub snippet: LiveBroadcastSnippet,
    pub content_details: LiveBroadcastContentDetails
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveBroadcastSnippet {
    pub resource_id: Id,
    #[serde(flatten)]
    pub inner: Snippet
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveBroadcastContentDetails {
    pub video_id: String,
    #[deprecated]
    pub start_at: Option<String>,
    #[deprecated]
    pub end_at: Option<String>,
    pub note: Option<String>,
    pub video_published_at: String
}
