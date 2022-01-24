use tokio::process::Command;

#[derive(Debug, Clone, Default)]
pub struct YoutubeDl;

impl YoutubeDl {
    pub async fn get_audio_stream_url(&self, id: &str) -> anyhow::Result<String> {
        let output = Command::new("youtube-dl")
            .arg("-g")
            .arg("-f")
            .arg("bestaudio")
            .arg(format!("https://www.youtube.com/watch?v={}", id))
            .output()
            .await?;

        if !output.status.success() {
            let err = String::from_utf8(output.stderr)?;
            anyhow::bail!("{}", err);
        }
        let url = String::from_utf8(output.stdout)?.trim().to_owned();
        Ok(url)
    }
}

#[cfg(test)]
mod test {
    use super::YoutubeDl;

    #[tokio::test]
    async fn stream_audio() {
        let youtube_dl = YoutubeDl::default();
        let video_ids = vec![
            "yfqTCWepx4U",
            "ZGIfJHeZKKE",
            "btecuyQKH-E",
            "uM7JjfHDuFM",
            "BgWpK28dt6I",
            "8xe6nLVXEC0",
            "O3WKbJLai1g",
        ];
        for video_id in video_ids {
            let url = youtube_dl.get_audio_stream_url(video_id).await;
            println!("{:?}", url);
            assert!(url.is_ok());
        }
    }
}
