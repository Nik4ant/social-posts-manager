pub mod reddit;

use std::{
    time::Duration,
};
use reqwest;


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
    let client = reqwest::Client::builder()
                                    .connect_timeout(Duration::from_secs(5))
                                    .timeout(Duration::from_secs(10))
                                    .connection_verbose(true)
                                    .https_only(true)
                                    // TODO: put custom errors for later. Commit working changes and get back to error later
                                    .build().unwrap();
    
    match destination {
        MediaSource::Reddit => {
            reddit::publish(&post, &client).await
                .unwrap_or_else(move |error| {
                    println!("Unexpected error occured:\n {}", error.to_string());
                });
        },
        _ => {

        }
    }
}