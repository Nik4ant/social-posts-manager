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
        title: "TESTING REDDIT API! (it works)".into(),
        // TODO: Add note somewhere that reddit markdown has nothing to do with Discord markdown. Use default one?
        content_markdown: "**LOL**, __LMAO__ <-- this text should be formated using markdown".into(),
        video_url: None,
    };
    media_poster::publish(post, MediaSource::Reddit).await;
}
