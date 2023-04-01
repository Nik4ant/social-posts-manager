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
// TODO: serialize this thing with serde
pub struct RedditPost {
    pub title: String,
    pub targeted_subreddits: Vec<String>,
    pub markdown_text: String,
}

#[derive(Debug)]
pub enum RedditSubmitResponse {
    Success {
        submition_link: String,
    },
    Failure {
        error_name: String,
        error_details: String,
    },
}
// NOTE: All this mess exists because Reddit API responds
// with HTML wrapped in json that doesn't make much sense. WTF?!
impl<'de> Deserialize<'de> for RedditSubmitResponse
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        // Represents not formatted response from Reddit API
        #[derive(Debug, Deserialize)]
        struct RedditRawSumbitResponse {
            jquery: Vec<Vec<serde_json::Value>>,
            success: bool
        }
        let submit_response: RedditRawSumbitResponse = Deserialize::deserialize(deserializer)?;
        // If success select submition link
        if submit_response.success {
            for item in submit_response.jquery {
                // Check if last item is an array with one String
                if let Some(serde_json::Value::Array(array)) = item.last() {
                    if array.len() != 1 { continue; }  // skiping garbage data

                    if let Some(serde_json::Value::String(str_value)) = array.first() {
                        if str_value.starts_with("https://") {
                            return Ok(RedditSubmitResponse::Success { 
                                submition_link: str_value.to_owned() 
                            });
                        }
                    }
                }
            }
        }
        // otherwise parse error info
        else {
            let items = submit_response.jquery;
            for (i, item) in items.iter().enumerate() {
                // Check if last item is an array with one String
                if let Some(serde_json::Value::Array(array)) = item.last() {
                    if array.len() != 1 { continue; }  // skiping garbage data
                    if let Some(serde_json::Value::String(error_name)) = array.first() {
                        // Start searching for text attribute as soon as error "code" was found.
                        // NOTE: There is no proper error structure, so this is the most 
                        // reliable way to detect stuff 
                        if error_name.starts_with(".error.") {
                            for j in i..items.len() {
                                if let Some(serde_json::Value::String(atr_value)) = items[j].last() {
                                    if atr_value == "text" {
                                        // If current attribute is "text" than next item is error details
                                        if let Some(serde_json::Value::Array(error_details_container)) = items[j + 1].last() {
                                            if let Some(serde_json::Value::String(error_details)) = error_details_container.first() {
                                                return Ok(RedditSubmitResponse::Failure { 
                                                    error_name: error_name.to_string(), 
                                                    error_details: error_details.to_string()
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                            break;
                        }
                    }
                }
            }
        }

        return Ok(RedditSubmitResponse::Failure { 
            error_name: "UNKNOWN".to_string(), 
            error_details: "Couldn't parse error info from API response!".to_string()
        });
    }
} 