use serde_json;
use serde::{
    self,
    Deserialize
};


#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum RedditAuthResponse {
    AuthData { access_token: String },
    ErrorData { error: String },
}

#[derive(Debug)]
pub struct RedditPost {
    pub title: String,
    pub targeted_subreddits: Vec<String>,
    pub markdown_text: String,
}

#[derive(Debug, Deserialize)]
// TODO: make it conditional, so if success is false than return an error and if it's okay return link
pub struct RedditSumbitResponse {
    // NOTE: All of this mess exists because Reddit API responds 
    // with HTML wrapped in json that looks wierd afterwards. WTF?!
    #[serde(deserialize_with = "deserialize_submit_link")]
    #[serde(alias = "jquery")]
    pub post_link: Option<String>,
    pub success: bool
}


fn deserialize_submit_link<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let submit_response: Vec<Vec<serde_json::Value>> = Deserialize::deserialize(deserializer)?;
    for item in submit_response {
        // Check if last item is an array with one String
        if let Some(serde_json::Value::Array(array)) = item.last() {
            if array.len() != 1 { continue; }  // skiping garbage data

            if let Some(serde_json::Value::String(str_value)) = array.first() {
                // Check if this is an error message, link to the post or just garbage data...
                // TODO: continue
                if str_value.starts_with("https://") {
                    return Ok(Some(str_value.to_owned()));
                }
            }
        }
    }

    return Ok(None);
    //return Ok(sumbit_link);
}