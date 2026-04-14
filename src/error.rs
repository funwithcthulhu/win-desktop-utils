#[derive(Debug)]
pub enum Error {
    Unsupported(&'static str),
    InvalidInput(&'static str),
    Io(std::io::Error),
    WindowsApi { context: &'static str, code: i32 },
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
