//! Implements the crate's error type

use std::{
    backtrace::Backtrace,
    error,
    fmt::{self, Display, Formatter},
    io,
};

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
impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        error!(with: error)
    }
}
impl From<feedme_feed::error::Error> for Error {
    fn from(error: feedme_feed::error::Error) -> Self {
        error!(with: error)
    }
}
impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        error!(with: error)
    }
}
impl From<time::error::Parse> for Error {
    fn from(error: time::error::Parse) -> Self {
        error!(with: error)
    }
}
