//! A `yt-dlp` interface

mod meta;

use crate::ytdlp::meta::{EntryMeta, Meta, PlaylistMeta};
use feedme_shared::{error, Entry, Error, Playlist, UuidBuilder};
use serde_json::Value;
use std::{collections::BTreeMap, fs, path::Path};
use time::{format_description::FormatItem, macros::format_description, Date};

/// Canonicalizes the metadata
pub fn canonicalize_meta() -> Result<(), Error> {
    // Process all metadata
    for (name, meta) in collect_metadata()? {
        // Select the appropriate translator
        match meta {
            Meta::Playlist(meta) => translate_playlist_meta(name, meta)?,
            Meta::Entry(meta) => translate_entry_meta(name, meta)?,
        }
    }
    Ok(())
}

/// Translate a playlist metadata file
fn translate_playlist_meta(name: String, meta: PlaylistMeta) -> Result<(), Error> {
    // Check if a thumbnail exists
    let basename = name.strip_suffix(".info.json").expect("invalid name of metadata file");
    let thumbnail_name = format!("{basename}.jpg");
    let maybr_thumbnail = match Path::new(&thumbnail_name).exists() {
        true => Some(thumbnail_name),
        false => None,
    };

    // Create the canonical representation
    let playlist = Playlist {
        title: meta.title,
        description: Some(meta.description),
        author: Some(meta.uploader),
        thumbnail: maybr_thumbnail,
        url: Some(meta.webpage_url),
    };

    // Serialize and write the metadata
    let playlist_json = serde_json::to_string_pretty(&playlist)?;
    fs::write("playlist-meta.feedme", playlist_json.as_bytes())?;
    Ok(())
}

/// Translates an entry metadata file
fn translate_entry_meta(name: String, meta: EntryMeta) -> Result<(), Error> {
    /// The date format within the metadata
    const DATE_FORMAT: &[FormatItem] = format_description!("[year][month][day]");

    // Build the video name
    let basename = name.strip_suffix(".info.json").expect("invalid name of metadata file");
    let video_name = format!("{basename}.{}", meta.ext);

    // Check if the entry exists already
    let entry_json_name = format!("playlist-entry{:05}.feedme", meta.playlist_index - 1);
    if Path::new(&entry_json_name).exists() {
        eprintln!("[feedme-ytdlp] Skipping existing entry: {video_name}");
        return Ok(());
    }

    // Get the file metadata and compute UUID
    eprintln!("[feedme-ytdlp] Computing UUID for: {video_name}");
    let file_meta = fs::metadata(&video_name)?;
    let file_uuid = UuidBuilder::new().context(&meta.id).finalize(&video_name)?;

    // Parse the date
    let date = Date::parse(&meta.upload_date, DATE_FORMAT)?;
    let date_unix_i64 = date.midnight().assume_utc().unix_timestamp();
    let date_unix = u64::try_from(date_unix_i64).map_err(|e| error!(with: e, "timestamp is too large"))?;

    // Build the entry
    let entry = Entry {
        file: video_name,
        size: file_meta.len(),
        type_: "video/mp4".to_string(),
        title: meta.title,
        description: Some(meta.description),
        uuid: file_uuid,
        duration: meta.duration,
        date: date_unix,
    };

    // Serialize and write the entry
    let entry_json = serde_json::to_string_pretty(&entry)?;
    fs::write(entry_json_name, entry_json.as_bytes())?;
    Ok(())
}

/// Reads and parses all `.info.json`-files from the current working directory
fn collect_metadata() -> Result<BTreeMap<String, Meta>, Error> {
    // Enumerate entries
    let mut entries = BTreeMap::new();
    'readdir: for entry in fs::read_dir(".")? {
        // Unwrap the entry
        let Ok(entry) = entry else {
            continue 'readdir;
        };

        // Check if the entry is not hidden and matches the pattern
        let Ok(entry_name) = entry.file_name().into_string() else {
            continue 'readdir;
        };
        if entry_name.starts_with('.') {
            continue 'readdir;
        }
        if !entry_name.ends_with(".info.json") {
            continue 'readdir;
        }

        // Read and parse the entry
        let json_raw = fs::read(entry.path())?;
        let json: Value = serde_json::from_slice(&json_raw)?;
        let meta = Meta::try_from(json)?;

        // Register the metadata
        entries.insert(entry_name, meta);
    }
    Ok(entries)
}
