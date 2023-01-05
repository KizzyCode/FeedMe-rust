//! An RSS podcast feed

mod helpers;
mod schema;

use crate::rss::{
    helpers::XmlWrite,
    schema::{Channel, Enclosure, Feed, Image, Item},
};
use feedme_shared::{error, Entry, Error, Playlist};
use std::{
    collections::BTreeSet,
    fs::{self, File},
    path::{Component, Path},
};
use xml::{EmitterConfig, EventWriter};

/// Builds a podcast feed from existing .feedme-metadata files
pub fn build_feed(base_url: &str, webroot: &str) -> Result<(), Error> {
    // Load the metadata
    let (playlist, entries) = collect_metadata()?;

    // Generate the thumbnail item
    let mut thumbnail = None;
    if let Some(thumbnail_) = playlist.thumbnail {
        let url = absolute_url(&thumbnail_, webroot, base_url)?;
        thumbnail = Some(Image { url });
    }

    // Serialize playlist
    let mut channel = Channel {
        title: playlist.title,
        itunes_type: "Serial".to_string(),
        link: playlist.url,
        itunes_author: playlist.author,
        description: playlist.description,
        itunes_image: thumbnail,
        items: Vec::new(),
    };

    // Serialize items
    for (index, entry) in entries.into_iter().enumerate() {
        // Build the enclosure entry referencing the file
        let enclosure = {
            let url = absolute_url(&entry.file, webroot, base_url)?;
            Enclosure { length: entry.size, type_: entry.type_, url }
        };

        // Create the playlist item
        let item = Item {
            title: entry.title,
            itunes_episode: (index as u64) + 1,
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

    // List all entry files and sort them
    let mut entry_names = BTreeSet::new();
    'list_dir: for file in fs::read_dir(".")? {
        // Unwrap the entry or skip it
        let Ok(file) = file else {
            continue 'list_dir;
        };

        // Get the filename and check that it references a feedme file
        let file_name_os = file.file_name();
        let Some(file_name) = file_name_os.to_str() else {
            continue 'list_dir;
        };

        // Ensure that the file is a playlist entry
        if !file_name.starts_with("playlist-entry") {
            continue 'list_dir;
        }
        if !file_name.ends_with(".feedme") {
            continue 'list_dir;
        }

        // Collect the entry
        entry_names.insert(file_name.to_string());
    }

    // Process the entries in order
    let mut entries = Vec::new();
    for entry_name in entry_names {
        // Parse the entry
        let entry_bin = fs::read(entry_name)?;
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
        return Err(error!("file is not within webroot: {}", canonical.display()));
    }

    // Create the relative path and the URL
    let relative_path = canonical.strip_prefix(webroot)?;
    let mut url_components = vec![base_url.to_string()];

    // Escape the individual path components
    for component in relative_path.components() {
        // Get the path component
        let Component::Normal(component) = component else {
            return Err(error!("unexpected path component: {component:?}"));
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
