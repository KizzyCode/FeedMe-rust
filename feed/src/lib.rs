#![doc = include_str!("../README.md")]

pub mod error;
pub mod rss;
pub mod uuid;

pub use crate::{
    rss::{Entry, Playlist, Rss},
    uuid::{Uuid, UuidBuilder},
};
