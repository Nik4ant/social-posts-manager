use std::{env};

use super::MediaSource;

use thiserror::Error;
use reqwest;

#[derive(Error, Debug)]
pub enum MediaPosterError {
    #[error("Can't process web request:\n`{source}`")]
    IO {
        #[from]
        source: reqwest::Error,
    },
    #[error("Can't process env variable:\n`{source}`")]
    Env {
        #[from]
        source: env::VarError,
    },
    #[error("Can't serialize post info for {media_source}; details:\n{error_details}")]
    Serialization {
        media_source: MediaSource,
        error_details: String
    },
    #[error("Can't auth to {0}; Error: {1}")]
    Auth(MediaSource, String),
    #[error("Can't submit post to {media_source}; {error_name:?} details:\n{error_details}")]
    Sumbition {
        media_source: MediaSource,
        error_name: String,
        error_details: String
    }
}