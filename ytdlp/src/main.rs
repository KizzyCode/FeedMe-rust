#![doc = include_str!("../README.md")]

mod ytdlp;

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
    // Get the argument and skip argv[0]
    let mut args = env::args().skip(1);
    if let Some(arg) = args.next() {
        return Err(error!("unexpected argument: {arg}"));
    }

    // Canonicalize metadata
    ytdlp::canonicalize_meta()
}

fn main() {
    // Print error information in case of a failure
    if let Err(e) = main_real() {
        exit_error(e);
    }
}
