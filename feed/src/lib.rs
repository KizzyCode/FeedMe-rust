#![doc = include_str!("../README.md")]

pub mod config;
pub mod error;
pub mod rss;
pub mod uuid;

pub use crate::{
    config::Config,
    rss::{Entry, Playlist, Rss},
    uuid::{Uuid, UuidBuilder},
};
