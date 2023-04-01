use std::{
    time::Duration,
};

mod media_poster;
use media_poster::*;

use dotenv;

#[tokio::main]
async fn main() {
    // TODO: configure cargo linter
    dotenv::dotenv().expect(".env must be present");

    let client = reqwest::Client::builder()
                .connect_timeout(Duration::from_secs(5))
                .timeout(Duration::from_secs(10))
                .connection_verbose(true)
                .https_only(true)
                .build().unwrap();

    let mastodon_post = MastodonPost { 
        content: "Just testing right now; This post should be private".to_string(), 
        visibility: MastodonVisibility::Private, 
        spoiler_text: Some("HUGE SPOILERS FOR GPT5".to_string()),
    };
    let reddit_post = RedditPost { 
        title: "Testing private posts".to_string(),
        markdown_text: "**YO!!!!** Private posts are working".to_string(),
        targeted_subreddits: vec!["r/Nik4anter_field".to_string()],
    };
    // TODO: handle error properly later
    // Finished refactoring (proper error messages)
    /*media_poster::reddit::publish(reddit_post, &client).await.unwrap_or_else(move |error| {
        println!("{}", error)
    });*/
    media_poster::mastodon::publish(mastodon_post, &client).await.unwrap_or_else(move |error| {
        println!("{}", error)
    });
}
