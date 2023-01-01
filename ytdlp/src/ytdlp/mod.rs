//! A `yt-dlp` interface

mod meta;

use crate::{
    error,
    error::Error,
    ytdlp::meta::{EntryMeta, Meta, PlaylistMeta},
};
use feedme_feed::{
    rss::{Entry, Playlist},
    uuid::UuidBuilder,
};
use serde_json::Value;
use std::{collections::BTreeMap, fs, path::Path};
use time::{format_description::FormatItem, macros::format_description, Date};

/// A `yt-dlp` interface
#[derive(Debug, Clone)]
pub struct YtDlp {
    /// The playlist URL
    playlist_url: String,
    /// Additional raw `yt-dlp` arguments
    raw_args: Vec<String>,
}
impl YtDlp {
    /// The yt-dlp download arguments
    const ARGS_DOWNLOAD: &[&'static str] = &[
        "--write-info-json",
        "--write-playlist-metafiles",
        "--restrict-filenames",
        "--format=bestvideo[ext=mp4][vcodec^=avc1]+bestaudio[ext=m4a]/best[ext=mp4][vcodec^=avc1]/best[ext=mp4]/best",
    ];

    /// Creates a new `yt-dlp` instance
    pub fn new<T>(playlist_url: T) -> Self
    where
        T: ToString,
    {
        Self { playlist_url: playlist_url.to_string(), raw_args: Vec::new() }
    }
    /// Sets raw args that are passed to `yt-dlp` upon invocation
    pub fn set_raw_args<I, IT>(&mut self, args: I)
    where
        I: IntoIterator<Item = IT>,
        IT: ToString,
    {
        let args_string = args.into_iter().map(|a| a.to_string());
        self.raw_args.extend(args_string);
    }

    /// Downloads a playlist into the destination directory
    pub fn download(&self) -> Result<Playlist, Error> {
        // Download the files
        self.fetch_files()?;

        // Build the playlist
        let meta = self.get_metadata()?;
        let entries = self.build_entries(&meta)?;
        self.build_playlist(&meta, entries)
    }

    /// Fetches the playlist files
    #[cfg(target_family = "unix")]
    fn fetch_files(&self) -> Result<(), Error> {
        use std::{
            io,
            os::fd::{AsRawFd, FromRawFd},
            process::{Command, Stdio},
        };

        // Pipe stdout and stderr to stderr
        // Note: This is safe, because `libc::dup` is a safe function and `Stdio::from_raw_fd` does not raise ownership
        // issues because it owns the duplicated file descriptor exclusively
        let stderr_fd = io::stderr().as_raw_fd();
        let stdout_stderr = unsafe {
            let stderr_fd = libc::dup(stderr_fd);
            Stdio::from_raw_fd(stderr_fd)
        };
        let stderr_stderr = unsafe {
            let stderr_fd = libc::dup(stderr_fd);
            Stdio::from_raw_fd(stderr_fd)
        };

        // Spawn yt-dlp
        let mut yt_dlp = Command::new("yt-dlp")
            .args(Self::ARGS_DOWNLOAD)
            .args(&self.raw_args)
            .arg(&self.playlist_url)
            .stdin(Stdio::null())
            .stdout(stdout_stderr)
            .stderr(stderr_stderr)
            .spawn()?;

        // Wait until `yt-dlp` is done
        let status = yt_dlp.wait()?;
        if !status.success() {
            return Err(error!("failed to download files"));
        }
        Ok(())
    }
    /// Fetches the playlist files
    #[cfg(not(target_family = "unix"))]
    fn fetch_files(&self) -> Result<(), Error> {
        compile_error!("Your platform is currently unsupported");
    }

    /// Builds a playlist object from the metadata and the given entries
    fn build_playlist(&self, meta: &BTreeMap<String, Meta>, entries: Vec<Entry>) -> Result<Playlist, Error> {
        // Get the playlist metadata entry
        let meta: &PlaylistMeta =
            meta.iter().find_map(|(_, meta)| meta.try_as()).ok_or(error!("missing playlist metadata JSON"))?;

        // Build the playlist
        let playlist = Playlist {
            title: meta.title.clone(),
            description: Some(meta.description.clone()),
            author: Some(meta.uploader.clone()),
            thumbnail: None,
            url: Some(meta.webpage_url.clone()),
            entries,
        };
        Ok(playlist)
    }

    /// Creates a sorted entry list from the metadata
    fn build_entries(&self, meta: &BTreeMap<String, Meta>) -> Result<Vec<Entry>, Error> {
        /// The date format within the metadata
        const DATE_FORMAT: &[FormatItem] = format_description!("[year][month][day]");

        // Enumerate all entries
        let mut entries = BTreeMap::new();
        'mapper: for (name, meta) in meta {
            // Skip non-`EntryMeta` entries
            let Some(meta) = meta.try_as::<EntryMeta>() else {
                continue 'mapper;
            };

            // Build the correct path
            let basename = name.strip_suffix(".info.json").expect("invalid name of metadata file");
            let video_name = format!("{basename}.mp4");
            let video_path = Path::new(&video_name).canonicalize()?;

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
                path: video_path,
                size: file_meta.len(),
                type_: "video/mp4".to_string(),
                title: meta.title.clone(),
                description: Some(meta.description.clone()),
                uuid: file_uuid,
                duration: meta.duration,
                date: date_unix,
            };
            entries.insert(meta.playlist_index, entry);
        }

        // Get a sorted array from all entries
        let entries = entries.into_values().collect();
        Ok(entries)
    }

    /// Reads and parses all `.info.json`-files from the current working directory
    fn get_metadata(&self) -> Result<BTreeMap<String, Meta>, Error> {
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
}
