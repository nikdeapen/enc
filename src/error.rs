use std::fmt::{Display, Formatter};
use std::io;
use std::io::ErrorKind;

/// An error processing encoded data.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// A data streaming error.
    Stream(io::Error),

    /// An integer overflowed.
    IntegerOverflow,

    /// The target buffer had insufficient space.
    InsufficientTargetSpace,

    /// The encoded data was invalid.
    ///
    /// This variant is `#[non_exhaustive]` so more context can be added to it without a breaking change.
    #[non_exhaustive]
    InvalidEncodedData {
        /// The underlying cause, when one is available.
        reason: Option<Box<dyn std::error::Error + Sync + Send>>,
    },
}

impl Error {
    //! Construction

    /// Creates an error for invalid encoded data.
    pub fn invalid_encoded_data() -> Self {
        Self::InvalidEncodedData { reason: None }
    }

    /// Creates an error for invalid encoded data with the underlying `reason`.
    pub fn invalid_encoded_data_with<R>(reason: R) -> Self
    where
        R: Into<Box<dyn std::error::Error + Sync + Send>>,
    {
        Self::InvalidEncodedData {
            reason: Some(reason.into()),
        }
    }
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

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Stream(error) => Some(error),
            Self::InvalidEncodedData { reason } => {
                reason.as_deref().map(|r| r as &dyn std::error::Error)
            }
            _ => None,
        }
    }
}
