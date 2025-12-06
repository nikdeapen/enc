use std::fmt::{Display, Formatter};
use std::io;
use std::io::ErrorKind;

/// An error processing encoded data.
#[derive(Debug)]
pub enum Error {
    /// A data streaming error.
    Stream(io::Error),

    /// An integer overflowed.
    IntegerOverflow,

    /// The target buffer had insufficient space.
    InsufficientTargetSpace,

    /// The encoded data was invalid.
    InvalidEncodedData {
        reason: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}

impl From<Error> for io::Error {
    fn from(error: Error) -> Self {
        match error {
            Error::Stream(error) => error,
            Error::IntegerOverflow => Self::new(ErrorKind::InvalidInput, error),
            Error::InsufficientTargetSpace => Self::new(ErrorKind::InvalidInput, error),
            Error::InvalidEncodedData { .. } => Self::new(ErrorKind::InvalidData, error),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::Stream(error)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stream(error) => write!(f, "{}", error),
            Self::IntegerOverflow => write!(f, "integer overflow"),
            Self::InsufficientTargetSpace => write!(f, "insufficient target space"),
            Self::InvalidEncodedData { reason } => {
                write!(f, "invalid encoded data")?;
                if let Some(reason) = reason {
                    write!(f, ": {reason}")?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for Error {}
