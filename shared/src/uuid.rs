//! A UUID

use crate::error::Error;
use blake2::{
    digest::{Update, VariableOutput},
    Blake2bVar,
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

/// A builder to create deterministic file-unique UUIDs
#[derive(Debug, Clone, Copy)]
pub struct UuidBuilder<'a> {
    /// The domain context for domain separation
    domain: Option<&'a [u8]>,
    /// The file specific context (e.g. an external file ID etc.)
    context: Option<&'a [u8]>,
}
impl<'a> UuidBuilder<'a> {
    /// The default domain
    pub const DEFAULT_DOMAIN: [u8; 16] = *b"\x85\xCA\x8F\x3A\x6A\xB5\x4F\x93\xA0\xAF\x99\x8E\xFE\x51\xC1\x55";
    /// The default context
    pub const DEFAULT_CONTEXT: [u8; 0] = *b"";

    /// Creates a new UUID builder
    pub const fn new() -> Self {
        Self { domain: None, context: None }
    }

    /// Sets the domain (defaults to `Self::DEFAULT_DOMAIN`)
    pub fn domain<T>(mut self, domain: &'a T) -> Self
    where
        T: AsRef<[u8]>,
    {
        self.domain = Some(domain.as_ref());
        self
    }
    /// Sets the context (defaults to `Self::DEFAULT_CONTEXT`)
    pub fn context<T>(mut self, context: &'a T) -> Self
    where
        T: AsRef<[u8]>,
    {
        self.context = Some(context.as_ref());
        self
    }

    /// Computes a UUID for the given file
    pub fn finalize<P>(self, file: P) -> Result<Uuid, Error>
    where
        P: AsRef<Path>,
    {
        // Initialize hasher
        let mut hasher = Blake2bVar::new(Uuid::SIZE)?;
        hasher.update(self.domain.unwrap_or(&Self::DEFAULT_DOMAIN));
        hasher.update(self.context.unwrap_or(&Self::DEFAULT_CONTEXT));

        // Ingest file
        let mut file = {
            let file = File::open(file)?;
            BufReader::new(file)
        };
        'read_file: loop {
            // Read chunk
            let data = file.fill_buf()?;
            if data.is_empty() {
                break 'read_file;
            }

            // Ingest chunk
            hasher.update(data);
            let len = data.len();
            file.consume(len);
        }

        // Compute the hash
        let mut bytes = [0; Uuid::SIZE];
        hasher.finalize_variable(&mut bytes)?;
        Ok(Uuid { bytes })
    }
}

/// A UUID
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Uuid {
    /// The UUID bytes
    pub bytes: [u8; Self::SIZE],
}
impl Uuid {
    /// The UUID size
    const SIZE: usize = 16;
}
impl Display for Uuid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Create a byte iterator
        let mut _bytes = self.bytes.iter();
        let bytes = &mut _bytes;

        // Write first segment
        bytes.take(4).try_for_each(|b| write!(f, "{:02X}", *b))?;
        write!(f, "-")?;

        // Write second segment
        bytes.take(2).try_for_each(|b| write!(f, "{:02X}", *b))?;
        write!(f, "-")?;

        // Write third segment
        bytes.take(2).try_for_each(|b| write!(f, "{:02X}", *b))?;
        write!(f, "-")?;

        // Write fourth segment
        bytes.take(2).try_for_each(|b| write!(f, "{:02X}", *b))?;
        write!(f, "-")?;

        // Write final segment
        bytes.take(6).try_for_each(|b| write!(f, "{:02X}", *b))?;
        Ok(())
    }
}
