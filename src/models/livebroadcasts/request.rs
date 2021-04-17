use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LiveBroadcastRequestBuilder {
    pub max_results: Option<u64>,
    pub mine: Option<bool>
}

impl LiveBroadcastRequestBuilder {
    pub(crate) fn build<S: Into<String>>(self, key : S) -> LiveBroadcastRequest {
        LiveBroadcastRequest {
            part: String::from("snippet,contentDetails"),
            key: key.into(),
            builder: self
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveBroadcastRequest {
    part: String,
    key: String,
    #[serde(flatten)]
    builder: LiveBroadcastRequestBuilder
}


#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatRequestBuilder {
    pub live_chat_id: Option<String>,
    pub page_token: Option<String>
}

impl LiveChatRequestBuilder {
    pub(crate) fn build<S: Into<String>>(self, key : S) -> LiveChatRequest {
        LiveChatRequest {
            part: String::from("snippet,authorDetails"),
            key: key.into(),
            builder: self
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveChatRequest {
    part: String,
    key: String,
    #[serde(flatten)]
    builder: LiveChatRequestBuilder
}


#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SendLiveChatRequestBuilder {
    pub live_chat_id: Option<String>,
}


impl SendLiveChatRequestBuilder {
    
    #[cfg(feature="post")]
    pub(crate) fn build<S: Into<String>>(self, key : S) -> SendLiveChatRequest {
        SendLiveChatRequest {
            part: String::from("snippet"),
            key: key.into(),
            builder: self
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendLiveChatRequest {
    part: String,
    key: String,    
    #[serde(flatten)]
    builder: SendLiveChatRequestBuilder
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SendLiveChatMessageDetails {
    pub message_text: Option<String>
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SendLiveChatRequestBody {
    pub snippet: SendLiveChatRequestBodyContents
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SendLiveChatRequestBodyContents {
    pub live_chat_id: Option<String>,
    #[serde(rename="type")]
    pub typed: Option<String>,
    pub text_message_details: SendLiveChatMessageDetails
}
