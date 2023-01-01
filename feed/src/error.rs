//! Implements the crate's error type

use std::{
    backtrace::Backtrace,
    convert::Infallible,
    error,
    fmt::{self, Display, Formatter},
    io,
    path::StripPrefixError,
    str::Utf8Error,
};

use blake2::digest::{InvalidBufferSize, InvalidOutputSize};
use time::error::{ComponentRange, Format};

/// Creates a new error
#[macro_export]
macro_rules! error {
    (with: $error:expr) => {{
        let error = $error.to_string();
        let source = Box::new($error);
        $crate::error::Error::new(error, Some(source))
    }};
    (with: $error:expr, $($arg:tt)*) => {{
        let error = format!($($arg)*);
        let source = Box::new($error);
        $crate::error::Error::new(error, Some(source))
    }};
    ($($arg:tt)*) => {{
        let error = format!($($arg)*);
        $crate::error::Error::new(error, None)
    }};
}

/// The crates error type
#[derive(Debug)]
pub struct Error {
    /// The error description
    pub error: String,
    /// The underlying error
    pub source: Option<Box<dyn std::error::Error + Send>>,
    /// The backtrace
    pub backtrace: Backtrace,
}
impl Error {
    /// Creates a new error
    #[doc(hidden)]
    pub fn new(error: String, source: Option<Box<dyn std::error::Error + Send>>) -> Self {
        let backtrace = Backtrace::capture();
        Self { error, source, backtrace }
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Print the error
        writeln!(f, "{}", self.error)?;

        // Print the source
        if let Some(source) = &self.source {
            writeln!(f, " caused by: {}", source)?;
        }
        Ok(())
    }
}
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        let source = self.source.as_ref()?;
        Some(source.as_ref())
    }
}
impl From<Utf8Error> for Error {
    fn from(error: Utf8Error) -> Self {
        error!(with: error)
    }
}
impl From<StripPrefixError> for Error {
    fn from(error: StripPrefixError) -> Self {
        error!(with: error)
    }
}
impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!("infallible can never be constructed");
    }
}
impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        error!(with: error)
    }
}
impl From<InvalidOutputSize> for Error {
    fn from(error: InvalidOutputSize) -> Self {
        error!("{error}")
    }
}
impl From<InvalidBufferSize> for Error {
    fn from(error: InvalidBufferSize) -> Self {
        error!("{error}")
    }
}
impl From<ComponentRange> for Error {
    fn from(error: ComponentRange) -> Self {
        error!(with: error)
    }
}
impl From<Format> for Error {
    fn from(error: Format) -> Self {
        error!(with: error)
    }
}
impl From<xml::writer::Error> for Error {
    fn from(error: xml::writer::Error) -> Self {
        error!(with: error)
    }
}
impl From<toml::de::Error> for Error {
    fn from(error: toml::de::Error) -> Self {
        error!(with: error)
    }
}
