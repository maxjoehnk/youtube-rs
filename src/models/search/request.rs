use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
pub struct SearchRequestBuilder {
    pub query: Option<String>,
    pub channel_id: Option<String>
}

impl SearchRequestBuilder {
    pub(crate) fn build<S: Into<String>>(self, api_key: S) -> SearchRequest {
        SearchRequest {
            part: String::from("snippet"),
            key: api_key.into(),
            query: self.query,
            channel_id: self.channel_id
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequest {
    part: String,
    key: String,
    #[serde(rename = "q")]
    query: Option<String>,
    channel_id: Option<String>
}