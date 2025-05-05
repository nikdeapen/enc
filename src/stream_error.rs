use std::fmt::{Debug, Display, Formatter};

/// A stream encoding error.
pub enum StreamError {
    /// A stream source error.
    Source(std::io::Error),

    /// An encoding error.
    Encoding(crate::Error),
}

impl From<std::io::Error> for StreamError {
    fn from(error: std::io::Error) -> Self {
        Self::Source(error)
    }
}

impl From<StreamError> for std::io::Error {
    fn from(error: StreamError) -> Self {
        match error {
            StreamError::Source(error) => error,
            StreamError::Encoding(error) => error.into(),
        }
    }
}

impl From<crate::Error> for StreamError {
    fn from(error: crate::Error) -> Self {
        Self::Encoding(error)
    }
}

impl Debug for StreamError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StreamError::Source(error) => write!(f, "{:?}", error),
            StreamError::Encoding(error) => write!(f, "{:?}", error),
        }
    }
}

impl Display for StreamError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StreamError::Source(error) => write!(f, "{}", error),
            StreamError::Encoding(error) => write!(f, "{}", error),
        }
    }
}
