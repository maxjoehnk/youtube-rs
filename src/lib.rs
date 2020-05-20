use crate::models::*;
use reqwest::get;
use tokio::process::Command;
use failure::format_err;

pub mod models;

pub struct YoutubeApi {}

impl YoutubeApi {
    pub async fn get_video_info(id: &str) -> Result<VideoMetadata, failure::Error> {
        let url = format!("https://www.youtube.com/get_video_info?video_id={}", id);
        let res = get(&url).await?.text().await?;
        let response: VideoMetadataResponse = serde_urlencoded::from_str(&res)?;
        let metadata: VideoMetadata = serde_json::from_str(&response.player_response)?;

        Ok(metadata)
    }

    pub async fn get_audio_stream_url(id: &str) -> Result<String, failure::Error> {
        let output = Command::new("youtube-dl").arg("-g").arg("-f").arg("bestaudio").arg(format!("https://www.youtube.com/watch?v={}", id)).output().await?;

        if !output.status.success() {
            let err = String::from_utf8(output.stderr)?;
            return Err(format_err!("{}", err));
        }
        let url = String::from_utf8(output.stdout)?.trim().to_owned();
        Ok(url)
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

    #[tokio::test]
    async fn stream_audio() {
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
            let url = YoutubeApi::get_audio_stream_url(video_id).await;
            println!("{:?}", url);
            assert!(url.is_ok());
        }
    }
}
