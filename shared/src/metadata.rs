//! Common metadata representation

use crate::Uuid;
use serde::{Deserialize, Serialize};

/// A playlist entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    /// The name of the referenced file
    pub file: String,
    /// The entry UUID
    pub uuid: Uuid,
    /// The size of the referenced file in bytes
    pub size: u64,
    /// The MIME type of the entry
    #[serde(rename = "type")]
    pub type_: String,
    /// The entry length in seconds
    pub duration: u64,
    /// The entry creation time as unix timestamp
    pub date: u64,
    /// The human readable entry title
    pub title: String,
    /// The item description
    pub description: Option<String>,
}

/// A playlist
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    /// The human readable playlist title
    pub title: String,
    /// The playlist description
    pub description: Option<String>,
    /// The playlist author
    pub author: Option<String>,
    /// The path to the image thumbnail
    pub thumbnail: Option<String>,
    /// The URL to the show
    pub url: Option<String>,
}
