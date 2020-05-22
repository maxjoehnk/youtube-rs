use failure::Error;
use oauth2::basic::BasicClient;
use reqwest::{Client, get, RequestBuilder, StatusCode};
use serde::Serialize;

use crate::models::*;
use crate::token::AuthToken;

const SEARCH_URL: &str = "https://www.googleapis.com/youtube/v3/search";
const LIST_PLAYLISTS_URL: &str = "https://www.googleapis.com/youtube/v3/playlists";

#[derive(Debug, Clone)]
pub(crate) struct YoutubeOAuth {
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
    pub(crate) client: BasicClient,
    pub(crate) token: AuthToken,
}

#[derive(Debug, Clone)]
pub struct YoutubeApi {
    pub(crate) api_key: String,
    pub(crate) oauth: Option<YoutubeOAuth>,
    pub(crate) client: Client,
}

mod auth;

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
            oauth: None,
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

    pub async fn list_playlists(&self, request: ListPlaylistsRequestBuilder) -> Result<ListPlaylistsResponse, failure::Error> {
        let request = request.build();
        let response = self.api_get(LIST_PLAYLISTS_URL, request)
            .await?
            .json()
            .await?;

        Ok(response)
    }

    async fn api_get<S: Into<String>, T: Serialize>(
        &self,
        url: S,
        params: T,
    ) -> Result<reqwest::Response, Error> {
        let req = self.client.get(&url.into()).query(&params);
        if let Some(oauth) = self.oauth.as_ref() {
            if oauth.token.requires_new_token().await {
                oauth.token.refresh(&oauth.client).await?;
            }
            let res = req
                .try_clone()
                .unwrap()
                .bearer_auth(oauth.token.get_auth_header().await?)
                .send()
                .await?
                .error_for_status();
            if let Err(err) = res {
                self.retry_request(req, err, oauth).await
            } else {
                let res = res.unwrap();
                Ok(res)
            }
        } else {
            let res = req.send().await?.error_for_status()?;
            Ok(res)
        }
    }

    async fn retry_request(
        &self,
        req: RequestBuilder,
        err: reqwest::Error,
        oauth: &YoutubeOAuth,
    ) -> Result<reqwest::Response, Error> {
        if let Some(StatusCode::UNAUTHORIZED) = err.status() {
            oauth.token.refresh(&oauth.client).await?;
            let res = req
                .bearer_auth(oauth.token.get_auth_header().await?)
                .send()
                .await?
                .error_for_status()?;
            Ok(res)
        } else {
            Err(err.into())
        }
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
