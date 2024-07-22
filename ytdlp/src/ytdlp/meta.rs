//! Metadata schemas

use crate::{error, error::Error};
use serde::Deserialize;
use serde_json::Value;

/// A playlist thumbnail item
#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct Thumbnail {
    /// The thumbnail URL
    pub url: String,
    /// The thumbnail heigth
    pub height: u64,
    /// The thumbnail width
    pub width: u64,
}

/// Playlist metadata
#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct PlaylistMeta {
    /// The human readable playlist title
    pub title: String,
    /// The playlist description
    pub description: String,
    /// The playlist uploader
    pub uploader: String,
    /// The playlist thumbnails
    pub thumbnails: Vec<Thumbnail>,
    /// The webpage URL
    pub webpage_url: String,
}

/// A playlist entry metadata
#[derive(Debug, Clone, Deserialize)]
pub struct EntryMeta {
    /// The youtube video ID
    pub id: String,
    /// The extension of the video file
    pub ext: String,
    /// The human readable video title
    pub title: String,
    /// The video description
    pub description: String,
    /// The video duration in seconds
    pub duration: u64,
    /// The upload date in `YYYYMMdd`
    pub upload_date: String,
    /// The index within the playlist (starting with 1)
    pub playlist_index: u64,
}

/// Some metadata
#[derive(Debug, Clone)]
pub enum Meta {
    /// Playlist associated metadata
    Playlist(PlaylistMeta),
    /// Playlist entry associated metadata
    Entry(EntryMeta),
}
impl TryFrom<Value> for Meta {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        /// A helper struct to extract the `_type` field
        #[derive(Debug, Deserialize)]
        struct TypeInfo {
            /// The `_type` field
            pub _type: String,
        }

        // Deserialize the value
        let type_info: TypeInfo = serde_json::from_value(value.clone())?;
        match type_info._type.as_str() {
            "playlist" => {
                let playlist_meta: PlaylistMeta = serde_json::from_value(value)?;
                Ok(Self::Playlist(playlist_meta))
            }
            "video" => {
                let entry_meta: EntryMeta = serde_json::from_value(value)?;
                Ok(Self::Entry(entry_meta))
            }
            other => Err(error!("unknown info-JSON type: {other}")),
        }
    }
}
