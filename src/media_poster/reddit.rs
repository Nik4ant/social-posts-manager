use std::{
    env,
    error::Error
};

use super::{PostInfo};

use reqwest::{
    self, 
    header::{
        HeaderMap, HeaderValue,
        USER_AGENT
    }
};
use serde::{
    self,
    Deserialize
};


#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum RedditAuthResponse {
    AuthData { access_token: String },
    ErrorData { error: String },
}

// TODO: docs
pub async fn publish(post: &PostInfo, client: &reqwest::Client) -> Result<(), Box<dyn Error>> {
    const BASE_API_URL: &str = "https://www.reddit.com/";
    const AUTH_API_URL: &str = "https://oauth.reddit.com";
    // Configures default headers
    let default_headers = HeaderMap::from_iter([
        (USER_AGENT, HeaderValue::from_str(&env::var("REDDIT_USERAGENT")?)?),
    ]);

    let client_login = env::var("REDDIT_CLIENT_ID")?;
    let client_password = env::var("REDDIT_CLIENT_SECRET")?;
    
    let auth_form = [
        ("grant_type", "password"),
        ("username", &env::var("REDDIT_USERNAME")?),
        ("password", &env::var("REDDIT_PASSWORD")?),
    ];
    let auth_response = client
        .post(format!("{BASE_API_URL:}/api/v1/access_token"))
        .headers(default_headers.clone())
        .basic_auth(client_login, Some(client_password))
        .form(&auth_form)
        .send()
        .await?;
    
    // println!("Full auth response:\n{}", auth_response.text().await?);
    let auth_json = auth_response.json::<RedditAuthResponse>().await?;

    // let auth_json = RedditAuthResponse::ErrorData { error: "123".to_string() };
    match auth_json {
        RedditAuthResponse::AuthData { access_token } => {
            println!("Access token: {}", access_token);
            
            // TODO: specify subreddits
            let target_subreddit = "r/Nik4anter_field";
            
            // NOTE: kind: one of (link, self, image, video, videogif)
            let post_form = [
                ("kind", "self"),
                ("title", &post.title),
                ("text", &post.content_markdown),
                ("sr", target_subreddit),
            ];
            
            let sumbit_response = client
                .post(format!("{AUTH_API_URL:}/api/submit"))
                .headers(default_headers.clone())
                .bearer_auth(access_token)
                .form(&post_form)
                .send()
                .await?;

            println!("post response:\n{}", sumbit_response.text().await?);
        },
        RedditAuthResponse::ErrorData { error } => {
            println!("Can't get access token from Reddit API:\n{}", error);
        }
    }

    return Ok(());
}