#![doc = include_str!("../README.md")]

pub mod error;
pub mod metadata;
pub mod uuid;

pub use crate::{
    error::Error,
    metadata::{Entry, Playlist},
    uuid::{Uuid, UuidBuilder},
};
