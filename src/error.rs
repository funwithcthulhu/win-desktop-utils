/// Error type for `win-desktop-utils`.
#[derive(Debug)]
pub enum Error {
    /// The requested operation is not implemented on the current platform or in the current crate version.
    Unsupported(&'static str),
    /// The caller supplied invalid input.
    InvalidInput(&'static str),
    /// A path was required to be absolute but was not.
    PathNotAbsolute,
    /// A required path does not exist.
    PathDoesNotExist,
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

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unsupported(msg) => write!(f, "unsupported operation: {msg}"),
            Self::InvalidInput(msg) => write!(f, "invalid input: {msg}"),
            Self::PathNotAbsolute => write!(f, "path must be absolute"),
            Self::PathDoesNotExist => write!(f, "path does not exist"),
            Self::Io(err) => write!(f, "I/O error: {err}"),
            Self::WindowsApi { context, code } => {
                write!(f, "Windows API error in {context} (code {code})")
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
