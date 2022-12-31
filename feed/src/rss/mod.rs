//! An RSS podcast feed

mod tags;
mod xml_utils;

use crate::{
    error::Error,
    rss::{
        tags::{Channel, Enclosure, Feed, Item},
        xml_utils::XmlWrite,
    },
    uuid::Uuid,
};
use xml::{EmitterConfig, EventWriter};

/// A playlist entry
#[derive(Debug, Clone)]
pub struct Entry {
    /// The path to the referenced file
    pub path: String,
    /// The size of the referenced file in bytes
    pub size: u64,
    /// The MIME type of the entry
    pub type_: String,
    /// The human readable entry title
    pub title: String,
    /// The item description
    pub description: Option<String>,
    /// The entry UUID
    pub uuid: Uuid,
    /// The entry length in seconds
    pub duration: u64,
    /// The entry creation time as unix timestamp
    pub date: u64,
}

/// A playlist
#[derive(Debug, Clone)]
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
    /// The playlist entries
    pub entries: Vec<Entry>,
}

/// A RSS podcast feed serializer
#[derive(Debug, Clone)]
pub struct Rss {
    /// The playlist
    playlist: Playlist,
}
impl Rss {
    /// Creates a new RSS podcast feed serializer for the given playlist
    pub const fn new(playlist: Playlist) -> Self {
        Self { playlist }
    }

    /// Serializes a playlist into an RSS podcast feed
    pub fn serialize(self) -> Result<String, Error> {
        // Serialize playlist
        let mut channel = Channel {
            title: self.playlist.title,
            link: self.playlist.url,
            itunes_author: self.playlist.author,
            description: self.playlist.description,
            itunes_image: self.playlist.thumbnail,
            items: Vec::new(),
        };

        // Serialize items
        for entry in self.playlist.entries {
            let enclosure = Enclosure { length: entry.size, type_: entry.type_, url: entry.path };
            let item = Item {
                itunes_title: entry.title,
                description: entry.description,
                enclosure,
                guid: entry.uuid,
                pub_date: entry.date,
                itunes_duration: entry.duration,
            };
            channel.items.push(item);
        }

        // Build the feed
        let feed_bin = {
            // Create the writer
            let mut feed_bin = Vec::new();
            let writer_config = EmitterConfig::new().perform_indent(true);
            let mut writer = EventWriter::new_with_config(&mut feed_bin, writer_config);

            // Write the feed
            let feed = Feed { channel };
            feed.write(&mut writer)?;
            feed_bin
        };

        // Convert the feed into a string
        let feed = String::from_utf8(feed_bin).expect("created non-UTF-8 RSS feed");
        Ok(feed)
    }
}
