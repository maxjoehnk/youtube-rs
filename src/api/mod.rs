use failure::Error;
use oauth2::basic::BasicClient;
use reqwest::{Client, get, RequestBuilder, StatusCode};
use serde::Serialize;

use crate::models::*;
use crate::token::AuthToken;
use serde::de::DeserializeOwned;

const SEARCH_URL: &str = "https://youtube.googleapis.com/youtube/v3/search";
const LIST_PLAYLISTS_URL: &str = "https://youtube.googleapis.com/youtube/v3/playlists";
const LIST_PLAYLIST_ITEMS_URL: &str = "https://youtube.googleapis.com/youtube/v3/playlistItems";
const LIVE_BROADCASTS: &str = "https://youtube.googleapis.com/youtube/v3/liveBroadcasts";
const LIVE_CHAT: &str = "https://youtube.googleapis.com/youtube/v3/liveChat/messages";

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
        let res = get(&url).await?.error_for_status()?.text().await?;
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
            .await?;

        YoutubeApi::handle_error(response).await
    }

    pub async fn list_playlists(&self, request: ListPlaylistsRequestBuilder) -> Result<ListPlaylistsResponse, failure::Error> {
        let request = request.build(&self.api_key);
        let response = self.api_get(LIST_PLAYLISTS_URL, request).await?;

        Ok(response)
    }

    pub async fn list_playlist_items(&self, request: ListPlaylistItemsRequestBuilder) -> Result<ListPlaylistItemsResponse, failure::Error> {
        let request = request.build(&self.api_key);
        let response = self.api_get(LIST_PLAYLIST_ITEMS_URL, request).await?;

        Ok(response)
    }

    pub async fn list_live_broadcasts(&self, request: LiveBroadcastRequestBuilder) 
        -> Result<ListLiveBroadcastResponse, failure::Error> {
        let request = request.build(&self.api_key);
        let response = self.api_get(LIVE_BROADCASTS, request)
            .await?;
        Ok(response)
    }

    pub async fn get_live_chat(&self, request: LiveChatRequestBuilder) 
        -> Result<LiveChatResponse, failure::Error> {
        let request = request.build(&self.api_key);
        let response = self.api_get(LIVE_CHAT, request)
            .await?;
        Ok(response)
    }

    async fn api_get<S: Into<String>, T: Serialize, TResponse: DeserializeOwned>(
        &self,
        url: S,
        params: T,
    ) -> Result<TResponse, Error> {
        let req = self.client.get(&url.into()).query(&params);
        let res = if let Some(oauth) = self.oauth.as_ref() {
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
            match res {
                Ok(res) => Ok(res),
                Err(err) => self.retry_request(req, err, oauth).await
            }
        } else {
            Ok(req.send().await?)
        }?;
        YoutubeApi::handle_error(res).await
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

    async fn handle_error<TResponse>(response: reqwest::Response) -> Result<TResponse, Error>
        where TResponse : DeserializeOwned
    {
        if response.error_for_status_ref().is_ok() {
            let res = response.text().await?;
            log::debug!("response text {}",res);
            let json = serde_json::from_str(&res)?;
            Ok(json)
        } else {
            let res = response.text().await?;
            log::error!("{:?}", res);
            let err: GoogleErrorResponse = serde_json::from_str(&res)?;
            log::error!("{:?}", err);
            Err(err.error.into())
        }
    }

    #[cfg(not(feature="post"))]
    pub async fn send_live_chat(&self, _request: SendLiveChatRequestBuilder, _body: SendLiveChatRequestBody) 
        -> Result<bool, failure::Error> {
            Err(failure::format_err!("feature disabled"))
    }

    #[cfg(feature="post")]
    pub async fn send_live_chat(&self, request: SendLiveChatRequestBuilder, body: SendLiveChatRequestBody) 
        -> Result<bool, failure::Error> {
        let request = request.build(&self.api_key);
        let response = self.api_post(LIVE_CHAT, request,body)
            .await?;
        Ok(response)
    }

    #[cfg(feature="post")]
    async fn api_post<S: Into<String>, T: Serialize, B: Serialize>(
        &self,
        url: S,
        params: T,
        body: B
    ) -> Result<bool, Error> {
        let req = self.client.post(&url.into()).query(&params).json(&body);
        let res = if let Some(oauth) = self.oauth.as_ref() {
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
            match res {
                Ok(res) => Ok(res),
                Err(err) => Err(err)
                
            }
        } else {
            Ok(req.send().await?)
        }?;
        YoutubeApi::handle_post_error(res).await
    }

    #[cfg(feature="post")]
    async fn handle_post_error(response: reqwest::Response) -> Result<bool, Error>
    {
        if response.error_for_status_ref().is_ok() {
            Ok(true)
        } else {
            let res = response.text().await?;
            println!("ERROR TEXT {}",res);
            let err: GoogleErrorResponse = serde_json::from_str(&res)?;
            log::error!("{:?}", err);

            Err(err.error.into())
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
