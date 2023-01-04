//! An RSS podcast feed

mod schema;
mod xml_utils;

use crate::rss::{
    schema::{Channel, Enclosure, Feed, Item},
    xml_utils::XmlWrite,
};
use feedme_shared::{error, Entry, Error, Playlist};
use std::{
    fs::{self, File},
    path::{Component, Path},
};
use xml::{EmitterConfig, EventWriter};

/// Builds a podcast feed from existing .feedme-metadata files
pub fn build_feed(base_url: &str, webroot: &str) -> Result<(), Error> {
    // Load the metadata
    let (playlist, entries) = collect_metadata()?;

    // Serialize playlist
    let thumbnail = match playlist.thumbnail {
        Some(thumbnail) => Some(absolute_url(&thumbnail, webroot, base_url)?),
        None => None,
    };
    let mut channel = Channel {
        title: playlist.title,
        link: playlist.url,
        itunes_author: playlist.author,
        description: playlist.description,
        itunes_image: thumbnail,
        items: Vec::new(),
    };

    // Serialize items
    for entry in entries {
        let enclosure = {
            let url = absolute_url(&entry.file, webroot, base_url)?;
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

    // Create the writer
    let file = File::create("feed.rss")?;
    let writer_config = EmitterConfig::new().perform_indent(true);
    let mut writer = EventWriter::new_with_config(file, writer_config);

    // Write the feed
    let feed = Feed { channel };
    feed.write(&mut writer)?;
    Ok(())
}

/// Collect all metadata files
fn collect_metadata() -> Result<(Playlist, Vec<Entry>), Error> {
    // Read the playlist
    let playlist_bin = fs::read("playlist-meta.feedme")?;
    let playlist = serde_json::from_slice(&playlist_bin)?;

    // Read the entries
    let mut entries = Vec::new();
    'read_entries: for index in 0.. {
        // Build the entry name and check if the entry exists
        let name = format!("playlist-entry{:05}.feedme", index);
        if !Path::new(&name).exists() {
            break 'read_entries;
        }

        // Parse the entry
        let entry_bin = fs::read(name)?;
        let entry = serde_json::from_slice(&entry_bin)?;
        entries.push(entry);
    }
    Ok((playlist, entries))
}

/// Creates an absolute URL for a file path
fn absolute_url(file: &str, webroot: &str, base_url: &str) -> Result<String, Error> {
    // Create the relative path
    let canonical = Path::new(file).canonicalize()?;
    if !canonical.starts_with(webroot) {
        return Err(error!("File is not within webroot: {}", canonical.display()));
    }

    // Create the relative path and the URL
    let relative_path = canonical.strip_prefix(webroot)?;
    let mut url_components = vec![base_url.to_string()];

    // Escape the individual path components
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
