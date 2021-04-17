use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListPlaylistsRequestBuilder {
    pub channel_id: Option<String>,
    #[serde(rename = "id")]
    pub playlist_id: Option<String>,
    pub max_results: Option<u64>,
    pub mine: Option<bool>
}

impl ListPlaylistsRequestBuilder {
    pub(crate) fn build<S: Into<String>>(self, key : S) -> ListPlaylistsRequest {
        ListPlaylistsRequest {
            part: String::from("snippet"),
            key: key.into(),
            builder: self
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPlaylistsRequest {
    part: String,
    key : String,
    #[serde(flatten)]
    builder: ListPlaylistsRequestBuilder
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListPlaylistItemsRequestBuilder {
    pub playlist_id: Option<String>,
    pub max_results: Option<u64>
}

impl ListPlaylistItemsRequestBuilder {
    pub(crate) fn build<S: Into<String>>(self, key : S) -> ListPlaylistItemsRequest {
        ListPlaylistItemsRequest {
            part: String::from("snippet,contentDetails"),
            key: key.into(),
            builder: self
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPlaylistItemsRequest {
    part: String,
    key: String,
    #[serde(flatten)]
    builder: ListPlaylistItemsRequestBuilder
}