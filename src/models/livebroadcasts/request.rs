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
