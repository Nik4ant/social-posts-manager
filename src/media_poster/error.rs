use std::env;

use super::MediaSource;

use thiserror::Error;
use reqwest;

#[derive(Error, Debug)]
pub enum MediaPosterError {
    #[error("Can't process web request")]
    IO(#[from] reqwest::Error),
    // TODO: Insert variable name into the error message somehow?
    #[error("Can't process env variable")]
    Env(#[from] env::VarError),
    #[error("Can't configure header values for web request")]
    HeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error("Can't auth to {0}; Error: {1}")]
    Auth(MediaSource, String),
    #[error("Can't submit post to {media_source}; {error_name:?} details:\n{error_details}")]
    Sumbition {
        media_source: MediaSource,
        error_name: String,
        error_details: String
    },
    #[error("unknown media_poster error; contact developer")]
    Unknown,
}