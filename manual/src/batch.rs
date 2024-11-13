//! Batch-processes the given files in the given order

use crate::meta;
use feedme_shared::{Error, Playlist};
use std::{collections::HashMap, fs};

/// Batch processes the given files in the given order and creates a playlist entry from the args
pub fn batch_process(args: HashMap<String, String>, files: Vec<String>) -> Result<(), Error> {
    // Process files
    for (index, file) in files.into_iter().enumerate() {
        // Create file entry
        write_entryinfo(index, file)?;
    }

    // Create playlist entry
    write_playlistinfo(args)?;
    Ok(())
}

/// Creates and writes the feedme playlist info
fn write_playlistinfo(mut args: HashMap<String, String>) -> Result<(), Error> {
    // Create playlist
    let title = args.remove("title").expect("Missing playlist title argument");
    let description = args.remove("description");
    let author = args.remove("author");
    let thumbnail = args.remove("thumbnail");
    let url = args.remove("url");
    let playlist = Playlist { title, description, author, thumbnail, url };

    // Serialize and write playlist to file
    let playlist_json = serde_json::to_string_pretty(&playlist)?;
    fs::write("playlist-meta.feedme", playlist_json.as_bytes())?;
    Ok(())
}

/// Computes and writes the feedme info for the given files
fn write_entryinfo(index: usize, file: String) -> Result<(), Error> {
    // Get and serialize entry
    let entry = meta::read_metadata(&file)?;
    let entry_json = serde_json::to_string_pretty(&entry)?;

    // Write entry to file
    let entry_json_name = format!("playlist-entry{:05}.feedme", index);
    fs::write(entry_json_name, entry_json.as_bytes())?;
    Ok(())
}