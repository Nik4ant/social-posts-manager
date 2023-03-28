#[allow(unused)]

mod media_poster;

use dotenv;
use media_poster::{
    PostInfo, MediaSource
};


#[tokio::main]
async fn main() {
    dotenv::dotenv().expect(".env must be present");

    let post = PostInfo {
        title: "SUPER HIT".into(),
        content_markdown: "LOL, LMAO".into(),
        video_url: None,
    };
    media_poster::publish(post, MediaSource::Reddit).await;
}
