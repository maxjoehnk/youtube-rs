use serde::{Serialize, Deserialize};

mod metadata;
mod search;

pub use self::metadata::*;
pub use self::search::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub kind: String,
    pub etag: String,
    #[serde(rename = "nextPageToken")]
    pub next_page_token: String,
    #[serde(rename = "prevPageToken")]
    pub prev_page_token: String,
    #[serde(rename = "regionCode")]
    pub region_code: String,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
    pub items: Vec<T>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageInfo {
    #[serde(rename = "totalResults")]
    pub total_results: u64,
    #[serde(rename = "resultsPerPage")]
    pub results_per_page: u64,
}
