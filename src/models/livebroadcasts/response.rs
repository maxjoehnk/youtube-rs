use serde::{Deserialize, Serialize};

use crate::models::{Response, Snippet};

pub type ListLiveBroadcastResponse = Response<LiveBroadcastResource>;
pub type LiveChatResponse = Response<LiveChatResource>;
pub type SendLiveChatResponse = Response<SendLiveChatResource>;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveBroadcastResource {
    pub kind: String,
    pub etag: String,
    pub id: String,
    pub snippet: LiveBroadcastSnippet,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveBroadcastSnippet {
    #[serde(flatten)]
    pub inner: Snippet,
    #[serde(flatten)]
    pub content_details: LiveBroadcastContentDetails
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveBroadcastContentDetails {
    pub scheduled_start_time: String,
    pub live_chat_id: String,
    pub actual_start_time: String,
    pub is_default_broadcast: bool
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatResource {
    pub kind: String,
    pub etag: String,
    pub snippet: LiveChatSnippet,
    pub author_details: AuthorDetails
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatSnippet {
    #[serde(flatten)]
    pub content_details: LiveChatContentDetails
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorDetails {
    pub display_name: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatContentDetails {
    pub display_message: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendLiveChatResource {
    
}