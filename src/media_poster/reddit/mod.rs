mod models;
// No need to export unnecessary inner details
use models::*;
pub use models::{RedditPost, RedditPostKind};

use std::{env};
use super::{MediaPosterError, MediaSource};
use reqwest::{
    self, 
    header::USER_AGENT
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


pub async fn publish(post: RedditPost, targeted_subreddits: Vec<String>, client: &reqwest::Client) -> Result<(), MediaPosterError> {
    // base url for APIs that require authecation
    const AUTH_API_URL: &str = "https://oauth.reddit.com";
    let reddit_app_user_agent = env::var("REDDIT_USERAGENT")?;
    
    let access_token = get_auth_token(&client, &reddit_app_user_agent).await?;
    // Serialize base post values to reuse params for post itself 
    // and only change "sr" (subreddit) value
    let mut post_form = serde_json::to_value(&post).map_err(move |error| {
        return MediaPosterError::Serialization { 
            media_source: MediaSource::Reddit,
            error_details: error.to_string(),
        };
    })?.as_object().expect("reddit post must be able to turn into map").to_owned();  // TODO: handle possible error properly

    // Sumbit post to each subreddit
    for target_subreddit in targeted_subreddits {
        post_form.insert("sr".to_string(), target_subreddit.into());

        // 1) POST request
        let sumbit_response = client
            .post(format!("{AUTH_API_URL:}/api/submit"))
            .header(USER_AGENT, &reddit_app_user_agent)
            .bearer_auth(&access_token)
            .form(&post_form)
            .send()
            .await?;
        // 2) Handle response
        let sumbit_json = sumbit_response.json::<RedditSubmitResponse>().await?;
        match sumbit_json {
            RedditSubmitResponse::Success { submition_link } => {
                println!("Submition link: `{}`", submition_link);
            },
            RedditSubmitResponse::Failure { error_name, error_details } => {
                return Err(MediaPosterError::Sumbition {
                    media_source: MediaSource::Reddit,
                    error_details: error_details,
                    error_name: error_name,
                });
            }
        }
    }

    return Ok(());
}