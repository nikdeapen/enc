use std::fmt::{Debug, Display, Formatter};

/// Either a stream error or an encoding error.
#[derive(Debug)]
pub enum StreamError {
    /// A stream error.
    Stream(std::io::Error),

    /// An encoding error.
    Encoding(crate::Error),
}

impl From<std::io::Error> for StreamError {
    fn from(error: std::io::Error) -> Self {
        Self::Stream(error)
    }
}

impl From<StreamError> for std::io::Error {
    fn from(error: StreamError) -> Self {
        match error {
            StreamError::Stream(error) => error,
            StreamError::Encoding(error) => error.into(),
        }
    }
}

impl From<crate::Error> for StreamError {
    fn from(error: crate::Error) -> Self {
        Self::Encoding(error)
    }
}

impl Display for StreamError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StreamError::Stream(error) => write!(f, "{}", error),
            StreamError::Encoding(error) => write!(f, "{}", error),
        }
    }
}

impl std::error::Error for StreamError {}
