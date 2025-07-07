use std::fmt::{Display, Formatter};
use std::io;
use std::io::ErrorKind;

/// An error processing encoded data.
#[derive(Debug)]
pub enum Error {
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
            Error::IntegerOverflow => io::Error::new(ErrorKind::OutOfMemory, error),
            Error::InsufficientTargetSpace => io::Error::new(ErrorKind::InvalidInput, error),
            Error::InvalidEncodedData { .. } => io::Error::new(ErrorKind::InvalidData, error),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message: &str = match self {
            Self::IntegerOverflow => "integer overflow",
            Self::InsufficientTargetSpace => "insufficient target space",
            Self::InvalidEncodedData { reason } => {
                write!(f, "invalid encoded data")?;
                if let Some(reason) = reason {
                    write!(f, ": {reason}")?;
                } else {
                    write!(f, "")?;
                }
                return Ok(());
            }
        };
        write!(f, "{message}")
    }
}

impl std::error::Error for Error {}
