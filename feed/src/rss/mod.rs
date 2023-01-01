//! An RSS podcast feed

mod feed;
mod xml_utils;

use crate::{
    error,
    error::Error,
    rss::{
        feed::{Channel, Enclosure, Feed, Item},
        xml_utils::XmlWrite,
    },
    uuid::Uuid,
    Config,
};
use std::path::{Component, Path, PathBuf};
use xml::{EmitterConfig, EventWriter};

/// A playlist entry
#[derive(Debug, Clone)]
pub struct Entry {
    /// The path to the referenced file
    pub path: PathBuf,
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
    /// The config
    config: Config,
    /// The playlist
    playlist: Playlist,
}
impl Rss {
    /// Creates a new RSS podcast feed serializer for the given playlist
    pub const fn new(config: Config, playlist: Playlist) -> Self {
        Self { config, playlist }
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
            let enclosure = {
                let url = Self::absolute_url(&self.config, &entry.path)?;
                Enclosure { length: entry.size, type_: entry.type_, url }
            };
            let item = Item {
                title: entry.title,
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

    /// Creates an absolute URL for a file path
    fn absolute_url(config: &Config, path: &Path) -> Result<String, Error> {
        // Create the relative path
        let canonical = path.canonicalize()?;
        if !canonical.starts_with(&config.webroot) {
            return Err(error!("file is not within webroot: {}", canonical.display()));
        }

        // Create the relative path and the URL
        let relative_path = canonical.strip_prefix(&config.webroot)?;
        let mut url_components = vec![config.base_url.to_string()];

        // Append the escaped path components to the URL
        for component in relative_path.components() {
            // Get the path component
            let Component::Normal(component) = component else {
                return Err(error!("unexpected path componend: {component:?}"));
            };

            // Escape the path component
            let component_str = component.to_str().ok_or(error!("path is not valid UTF-8"))?;
            let component = urlencoding::encode(component_str);
            url_components.push(component.to_string());
        }

        // Join the URL components
        let url = url_components.join("/");
        Ok(url)
    }
}
