#![doc = include_str!("../README.md")]

pub mod error;
pub mod metadata;
pub mod uuid;

pub use crate::error::Error;
pub use crate::metadata::{Entry, Playlist};
pub use crate::uuid::{Uuid, UuidBuilder};
