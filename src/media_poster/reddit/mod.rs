mod models;
// No need to export unnecessary inner details
use models::*;
pub use models::RedditPost;

use std::{env};
use super::{MediaPosterError, MediaSource};
use reqwest::{
    self, 
    header::{USER_AGENT}
};

/// Returns bearer auth token for Reddit API
async fn get_auth_token(client: &reqwest::Client, reddit_app_user_agent: &String) -> Result<String, MediaPosterError> {
    const BASE_API_URL: &str = "https://www.reddit.com/";

    let client_login = env::var("REDDIT_CLIENT_ID")?;
    let client_password = env::var("REDDIT_CLIENT_SECRET")?;

    let auth_form = [
        ("grant_type", "password"),
        ("username", &env::var("REDDIT_USERNAME")?),
        ("password", &env::var("REDDIT_PASSWORD")?),
    ];
    let auth_response = client
        .post(format!("{BASE_API_URL:}/api/v1/access_token"))
        .header(USER_AGENT, reddit_app_user_agent)
        .basic_auth(client_login, Some(client_password))
        .form(&auth_form)
        .send()
        .await?;
    
    let auth_json = auth_response.json::<RedditAuthResponse>().await?;
    match auth_json {
        RedditAuthResponse::AuthData { access_token } => {
            return Ok(access_token);
        },
        RedditAuthResponse::ErrorData { error } => {
            return Err(MediaPosterError::Auth(MediaSource::Reddit, error));
        }
    }
}

// TODO: docs
pub async fn publish(post: RedditPost, client: &reqwest::Client) -> Result<(), MediaPosterError> {
    // base url for APIs that require authecation
    const AUTH_API_URL: &str = "https://oauth.reddit.com";
    // 
    let reddit_app_user_agent = env::var("REDDIT_USERAGENT")?;

    let access_token = get_auth_token(&client, &reddit_app_user_agent).await?;

    for target_subreddit in post.targeted_subreddits {
        let post_form = [
            // TODO: proper enum value from post.kind
            ("kind", "self"),
            ("title", &post.title),
            ("text", &post.markdown_text),
            ("sr", &target_subreddit)
        ];
        
        let sumbit_response = client
            .post(format!("{AUTH_API_URL:}/api/submit"))
            .header(USER_AGENT, &reddit_app_user_agent)
            .bearer_auth(&access_token)
            .form(&post_form)
            .send()
            .await?;
        let sumbit_json = sumbit_response.json::<RedditSumbitResponse>().await?;
        match sumbit_json.post_link {
            Some(link) => {
                println!("Success: `{}`; Post link: `{}`", sumbit_json.success, link);
            },
            None => {
                // TODO:
                return Err(MediaPosterError::Sumbition(MediaSource::Reddit, "Can't parse error message yet".to_string()))
            }
        }
    }

    return Ok(());
}