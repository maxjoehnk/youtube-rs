use reqwest::{Client, get};

use crate::models::*;

const SEARCH_URL: &str = "https://www.googleapis.com/youtube/v3/search";

pub struct YoutubeApi {
    api_key: String,
    client: Client,
}

impl YoutubeApi {
    pub async fn get_video_info(id: &str) -> Result<VideoMetadata, failure::Error> {
        let url = format!("https://www.youtube.com/get_video_info?video_id={}", id);
        let res = get(&url).await?.text().await?;
        let response: VideoMetadataResponse = serde_urlencoded::from_str(&res)?;
        let metadata: VideoMetadata = serde_json::from_str(&response.player_response)?;

        Ok(metadata)
    }

    pub fn new<S: Into<String>>(api_key: S) -> Self {
        YoutubeApi {
            api_key: api_key.into(),
            client: Client::new(),
        }
    }

    pub async fn search(&self, search_request: SearchRequestBuilder) -> Result<SearchResponse, failure::Error> {
        let request = search_request.build(&self.api_key);
        let response = self.client.get(SEARCH_URL)
            .query(&request)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(response)
    }
}

#[cfg(test)]
mod test {
    use crate::models::SearchRequestBuilder;
    use crate::YoutubeApi;

    fn create_api() -> YoutubeApi {
        YoutubeApi::new(env!("YOUTUBE_API_KEY"))
    }

    #[tokio::test]
    async fn get_video_info() {
        let video_ids = vec![
            "yfqTCWepx4U",
            "ZGIfJHeZKKE",
            "btecuyQKH-E",
            "uM7JjfHDuFM",
            "BgWpK28dt6I",
            "8xe6nLVXEC0",
            "O3WKbJLai1g"
        ];
        for video_id in video_ids {
            let metadata = YoutubeApi::get_video_info(video_id).await;
            if metadata.is_err() {
                println!("{:?}", metadata);
            }
            assert!(metadata.is_ok());
        }
    }

    #[tokio::test]
    async fn search_should_work() {
        let api = create_api();
        let request = SearchRequestBuilder {
            query: Some(String::from("Don't stay in school")),
            ..SearchRequestBuilder::default()
        };

        let res = api.search(request).await;

        println!("{:?}", res);
        assert!(res.is_ok())
    }
}
