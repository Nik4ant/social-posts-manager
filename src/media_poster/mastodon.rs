use std::{
    env,
    fmt,
};
use super::MediaPosterError;
use reqwest::{self, header::ACCEPT};


#[derive(Debug)]
pub enum MastodonVisibility {
    Private,
    Unlisted,
    Public,
    Direct
}
impl fmt::Display for MastodonVisibility {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug)]
pub struct MastodonPost {
    pub content: String,
    pub visibility: MastodonVisibility,
    /// Warning text that will be shown before the actual post
    pub spoiler_text: Option<String>,
}


pub async fn publish(post: MastodonPost, client: &reqwest::Client) -> Result<(), MediaPosterError> {
    let base_url = env::var("MASTODON_INSTANCE_URL")?;
    let access_token = env::var("MASTODON_ACCESS_TOKEN")?;

    // Creating a status (on Mastodon status = post on your profile)
    let mut status_form = vec![
        ("status", post.content),
        ("visibility", post.visibility.to_string().to_lowercase()),
    ];
    if let Some(spoiler_text) = post.spoiler_text {
        status_form.push(("spoiler_text", spoiler_text));
    }

    let sumbition_response = client
        .post(format!("{base_url:}/api/v1/statuses"))
        .form(&status_form)
        .header(ACCEPT, "application/json")
        .bearer_auth(access_token)
        .send()
        .await?;

    println!("Mastodon response{}", sumbition_response.text().await?);

    return Ok(());
}