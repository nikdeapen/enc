use crate::Error;
use std::io::Read;

/// A value that can decode itself from a `Read`.
pub trait DecodeFromRead: Sized {
    /// Decodes a value from the `Read`.
    ///
    /// # Note
    /// The `Read` should contain only the decoded value and the implementation should fully drain
    /// the `Read` but this is not guaranteed if the value is improperly encoded.
    fn decode_from_read<R>(r: &mut R) -> Result<Self, Error>
    where
        R: Read;
}
