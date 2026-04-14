/// Error type for `win-desktop-utils`.
#[derive(Debug)]
pub enum Error {
    /// The requested operation is not implemented on the current platform or in the current crate version.
    Unsupported(&'static str),
    /// The caller supplied invalid input.
    InvalidInput(&'static str),
    /// An underlying I/O operation failed.
    Io(std::io::Error),
    /// A Windows API call failed.
    WindowsApi {
        /// Short label describing the failing API call.
        context: &'static str,
        /// Raw Windows error or return code when available.
        code: i32,
    },
}

/// Convenient result alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
