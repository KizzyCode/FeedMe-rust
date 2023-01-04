#![doc = include_str!("../README.md")]

mod rss;

use feedme_shared::{error, Error};
use std::{env, process};

/// Displays the error and exits with status `2`
fn exit_error(e: Error) -> ! {
    // Print the error
    eprintln!("Fatal error: {e}");

    // Print the backtrace if any
    if e.has_backtrace() {
        eprintln!();
        eprintln!("{}", e.backtrace);
    }

    // Print general help
    eprintln!("---");
    eprint!("{}", include_str!("../HELP.txt"));
    process::exit(1);
}

/// The fallible, real main function
fn main_real() -> Result<(), Error> {
    // Load the required variables from the environment
    let Ok(base_url) = env::var("FEEDME_BASE_URL") else {
        return Err(error!("missing FEEDME_BASE_URL environment variable"));
    };
    let Ok(webroot) = env::var("FEEDME_WEBROOT") else {
        return Err(error!("missing FEEDME_WEBROOT environment variable"));
    };

    // Canonicalize metadata
    rss::build_feed(&base_url, &webroot)
}

fn main() {
    // Print error information in case of a failure
    if let Err(e) = main_real() {
        exit_error(e);
    }
}
