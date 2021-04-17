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
    #[serde(flatten)]
    pub author_details_id: AuthorDetailsId,
    pub is_chat_moderator: bool,
    pub is_chat_owner: bool,    
}

///
/// AuthorDetailsId contains just the display name and
/// the channel ID in a hashable structure.
/// This makes it easy to store a hash that uniquely identifies
/// a person based on their current display name. This means
/// that an application doesn't track display name changes.
/// If an application wants to track display name changes then 
/// use just the channel ID
/// (and deal with any privacy concerns associated)
///
#[derive(Clone, Debug, Serialize, Deserialize, Hash)]
#[serde(rename_all = "camelCase")]
pub struct AuthorDetailsId {
    pub display_name: String,
    pub channel_id: String,
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