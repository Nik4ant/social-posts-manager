use reqwest;

pub mod reddit;

// TODO: continue
pub struct PostInfo {
    pub title: String,
    pub content_markdown: String,
    pub video_url: Option<String>,
}
pub enum MediaSource {
    Discord,
    Twitter,
    Reddit,
    YouTubeCommunity,
    Mastodon,

    All
}

pub async fn publish(post: PostInfo, destination: MediaSource) {
    let client = reqwest::Client::new();
    
    match destination {
        MediaSource::Reddit => {
            reddit::publish(&post, &client)
                .await
                .unwrap_or_else(move |error| {
                    println!("Unexpected error occured:\n {}", error.to_string());
                });
        },
        _ => {

        }
    }
}