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
    pub(crate) fn build(self) -> ListPlaylistsRequest {
        ListPlaylistsRequest {
            part: String::from("snippet"),
            builder: self
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPlaylistsRequest {
    part: String,
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
    pub(crate) fn build(self) -> ListPlaylistItemsRequest {
        ListPlaylistItemsRequest {
            part: String::from("snippet,contentDetails"),
            builder: self
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPlaylistItemsRequest {
    part: String,
    #[serde(flatten)]
    builder: ListPlaylistItemsRequestBuilder
}
