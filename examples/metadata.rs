use youtube_api::{YoutubeApi};

#[tokio::main]
pub async fn main() {
    let metadata = YoutubeApi::get_video_info("O3WKbJLai1g").await.unwrap();

    println!("{:?}", metadata);
}
