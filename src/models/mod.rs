use serde::{Serialize, Deserialize};

mod metadata;
mod search;

pub use self::metadata::*;
pub use self::search::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<T> {
    pub kind: String,
    pub etag: String,
    pub next_page_token: Option<String>,
    pub prev_page_token: Option<String>,
    pub region_code: String,
    pub page_info: PageInfo,
    pub items: Vec<T>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub total_results: u64,
    pub results_per_page: u64,
}
