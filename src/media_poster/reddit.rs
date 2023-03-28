use std::{
    env,
    error::Error
};

use crate::media_poster::PostInfo;

use reqwest::{
    self, 
    header::{USER_AGENT}
};
use serde::{
    self,
    Deserialize
};


#[derive(Deserialize, Debug)]
#[serde(untagged)]
// TODO: continue
enum RedditAuthResponse {
    AuthData { access_token: String },
    ErrorData { error: String },
}

pub async fn publish(post: &PostInfo, client: &reqwest::Client) -> Result<(), Box<dyn Error>> {
    const BASE_URL: &str = "https://www.reddit.com/";
    // Thanks to https://github.com/halcyonnouveau/roux/blob/master/src/lib.rs#L126

    let client_login = env::var("REDDIT_CLIENT_ID")?;
    let client_password = env::var("REDDIT_CLIENT_SECRET")?;

    let form = [
        ("grant_type", "password"),
        ("username", &env::var("REDDIT_USERNAME")?),
        ("password", &env::var("REDDIT_PASSWORD")?),
    ];

    let response = client
        .post(format!("{BASE_URL:}/api/v1/access_token"))
        .header(USER_AGENT, env::var("REDDIT_USERAGENT")?)
        .basic_auth(client_login, Some(client_password))
        .form(&form)
        .send()
        .await?;

    let auth_response = response.json::<RedditAuthResponse>().await?;
    match auth_response {
        RedditAuthResponse::AuthData { access_token } => {
            println!("Access token: {}", access_token);
        },
        RedditAuthResponse::ErrorData { error } => println!("Can't get access token from Reddit API:\n{}", error)
    }

   return Ok(());
}