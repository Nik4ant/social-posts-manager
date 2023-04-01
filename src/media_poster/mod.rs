mod error;
pub use error::*;

// use statements are added just for convenience
pub mod reddit;
pub use reddit::*;

pub mod mastodon;
pub use mastodon::{MastodonPost, MastodonVisibility};

use std::{fmt};

#[derive(Debug)]
pub enum MediaSource {
    Twitter,
    Reddit,
    YouTubeCommunity,
    Mastodon,
}
impl fmt::Display for MediaSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}