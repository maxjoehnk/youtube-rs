use crate::models::*;
use reqwest::get;

pub mod models;
mod youtube_dl;

pub use youtube_dl::YoutubeDl;

pub struct YoutubeApi {}

impl YoutubeApi {
    pub async fn get_video_info(id: &str) -> Result<VideoMetadata, failure::Error> {
        let url = format!("https://www.youtube.com/get_video_info?video_id={}", id);
        let res = get(&url).await?.text().await?;
        let response: VideoMetadataResponse = serde_urlencoded::from_str(&res)?;
        let metadata: VideoMetadata = serde_json::from_str(&response.player_response)?;

        Ok(metadata)
    }
}

#[cfg(test)]
mod test {
    use crate::YoutubeApi;

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
}
