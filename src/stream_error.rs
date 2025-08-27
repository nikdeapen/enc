use crate::Error;
use std::fmt::{Debug, Display, Formatter};
use std::{fmt, io};

/// Either a source error or an encoding error.
#[derive(Debug)]
pub enum StreamError {
    /// A source error.
    Source(io::Error),

    /// An encoding error.
    Encoding(Error),
}

impl From<io::Error> for StreamError {
    fn from(error: io::Error) -> Self {
        Self::Source(error)
    }
}

impl From<StreamError> for io::Error {
    fn from(error: StreamError) -> Self {
        match error {
            StreamError::Source(error) => error,
            StreamError::Encoding(error) => error.into(),
        }
    }
}

impl From<Error> for StreamError {
    fn from(error: Error) -> Self {
        Self::Encoding(error)
    }
}

impl Display for StreamError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            StreamError::Source(error) => write!(f, "{error}"),
            StreamError::Encoding(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for StreamError {}
