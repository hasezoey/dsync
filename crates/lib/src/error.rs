// TODO: change backtrace implementation to be by thiserror, if possible once features become stable
// error_generic_member_access https://github.com/rust-lang/rust/issues/99301
// provide_any https://github.com/rust-lang/rust/issues/96024

use std::{backtrace::Backtrace, io::Error as ioError, path::Path};

pub type Result<T> = std::result::Result<T, Error>;

/// Error type for libytdlr, contains a backtrace, wrapper around [ErrorInner]
#[derive(Debug)]
pub struct Error {
    /// The actual error
    source: ErrorEnum,
    #[cfg(feature = "backtrace")]
    /// The backtrace for the error
    backtrace: Backtrace,
}

impl Error {
    /// Construct a new [Error] instance based on [ErrorInner]
    pub fn new(source: ErrorEnum) -> Self {
        return Self {
            source,
            #[cfg(feature = "backtrace")]
            backtrace: Backtrace::capture(),
        };
    }

    #[cfg(feature = "backtrace")]
    /// Get the backtrace that is stored
    pub fn get_backtrace(&self) -> &Backtrace {
        return &self.backtrace;
    }

    /// Create a custom [ioError] with this [Error] wrapped around with a [Path] attached
    pub fn custom_ioerror_path<M, P>(kind: std::io::ErrorKind, msg: M, path: P) -> Self
    where
        M: Into<String>,
        P: AsRef<Path>,
    {
        return Self::new(ErrorEnum::IoError(
            ioError::new(kind, msg.into()),
            format_path(path.as_ref().to_string_lossy().to_string()),
        ));
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return self.source.fmt(f);
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        return self.source.source();
    }
}

// implement all From<> variants that ErrorInner also implements
impl<T> From<T> for Error
where
    T: Into<ErrorEnum>,
{
    fn from(value: T) -> Self {
        return Self::new(value.into());
    }
}

/// Error type for "yt-downloader-rust", implements all Error types that could happen in this lib
#[derive(thiserror::Error, Debug)]
pub enum ErrorEnum {
    /// Wrapper Variant for [`std::io::Error`]
    /// Argument 1 (String) is up to the implementation to set, commonly the path
    #[error("IoError: {0}; {1}")]
    IoError(std::io::Error, String),
}

/// Helper function to keep consistent formatting
#[inline]
fn format_path(msg: String) -> String {
    return format!("Path \"{}\"", msg);
}
