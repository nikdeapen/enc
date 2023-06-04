use std::fmt::{Display, Formatter};

/// An error processing encoded data.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Error {
    /// An integer overflowed while computing a buffer size.
    IntegerOverflow,

    /// The target had insufficient space for the operation.
    InsufficientTargetSpace,
}

impl Error {
    //! Display

    /// Gets the error message.
    pub const fn message(&self) -> &'static str {
        match self {
            Self::IntegerOverflow => "integer overflow",
            Self::InsufficientTargetSpace => "insufficient target space",
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}
