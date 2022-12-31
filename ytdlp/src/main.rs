#![doc = include_str!("../README.md")]

mod error;

use feedme_feed::Rss;
use feedme_ytdlp::{error::Error, YtDlp};
use std::{backtrace::BacktraceStatus, env, process};

/// Displays the help and exits with status `1`
fn exit_help() -> ! {
    // Print help
    eprintln!("Usage: feedme-ytdlp <url> [<raw-args>...]");
    eprintln!();
    eprintln!("  url         The URL of the YouTube playlist to download");
    eprintln!("  raw-args    Zero or more raw arguments that will be passed to `yt-dlp` directly");
    eprintln!();
    process::exit(1);
}
/// Displays the error and exits with status `2`
fn exit_error(e: Error) -> ! {
    // Print the error
    eprintln!("Fatal error: {e}");

    // Print the backtrace if any
    if e.backtrace.status() == BacktraceStatus::Captured {
        eprintln!();
        eprintln!("{}", e.backtrace);
    }
    process::exit(2);
}

/// The fallible, real main function
fn main_real() -> Result<(), Error> {
    // Get the argument and skip argv[0]
    let mut args = env::args().skip(1);

    // Get the URL or display the help
    let Some(playlist_url) = args.next() else {
        exit_help();
    };

    // Download playlist
    let mut yt_dlp = YtDlp::new(playlist_url);
    yt_dlp.set_raw_args(args);
    let playlist = yt_dlp.download()?;

    // Create feed
    let feed = Rss::new(playlist).serialize()?;
    println!("{feed}");
    Ok(())
}

fn main() {
    // Print error information in case of a failure
    if let Err(e) = main_real() {
        exit_error(e);
    }
}
