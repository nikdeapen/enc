use std::fmt::{Display, Formatter};
use std::io;
use std::io::ErrorKind;

/// An error processing encoded data.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Error {
    /// An integer overflowed.
    IntegerOverflow,

    /// The target buffer had insufficient space for the operation.
    InsufficientTargetSpace,

    /// The encoded data was invalid.
    InvalidEncodedData,
}

impl Error {
    //! Display

    /// Gets the error message.
    pub const fn message(&self) -> &'static str {
        match self {
            Self::IntegerOverflow => "integer overflow",
            Self::InsufficientTargetSpace => "insufficient target space",
            Self::InvalidEncodedData => "invalid encoded data",
        }
    }
}

impl From<Error> for io::Error {
    fn from(error: Error) -> Self {
        match error {
            Error::IntegerOverflow => io::Error::new(ErrorKind::OutOfMemory, error),
            Error::InsufficientTargetSpace => io::Error::new(ErrorKind::InvalidInput, error),
            Error::InvalidEncodedData => io::Error::new(ErrorKind::InvalidData, error),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for Error {}
