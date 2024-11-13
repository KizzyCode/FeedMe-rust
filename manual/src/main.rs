#![doc = include_str!("../README.md")]

mod batch;
mod meta;

use feedme_shared::{error, Error};
use std::{collections::HashMap, env, process};

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
    // Parse arguments
    let mut args = HashMap::new();
    let mut files = Vec::new();
    for arg in env::args().skip(1) {
        // Check if we have a key-value arg
        if let Some(kv_arg) = arg.strip_prefix("--") {
            // Split argument
            let (key, value) = kv_arg.split_once('=').unwrap_or((kv_arg, ""));
            args.insert(key.to_string(), value.to_string());
        } else {
            // Add file to process
            files.push(arg);
        }
    }

    // Check that the required args exists
    if !args.contains_key("title") {
        // Fail with an error
        exit_error(error!(r#"Missing required argument: "--title=""#));
    }

    // Create metadata files
    batch::batch_process(args, files)
}

fn main() {
    // Print error information in case of a failure
    if let Err(e) = main_real() {
        exit_error(e);
    }
}
