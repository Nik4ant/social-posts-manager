use std::{env};

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
    // TODO: Check response and return Sumbition if smth wrong
    #[error("Can't sumbit post to {0}; Error {1}")]
    Sumbition(MediaSource, String),
    #[error("unknown media_poster error; contact developer")]
    Unknown,
}