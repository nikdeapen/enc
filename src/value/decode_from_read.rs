use crate::StreamError;
use std::io::Read;

/// A value that can decode itself from a `Read`.
pub trait DecodeFromRead: Sized {
    /// Decodes a value from the `Read`.
    ///
    /// # Note
    /// The implementation must fully drain the `Read`.
    fn decode_from_read<R>(r: &mut R) -> Result<Self, StreamError>
    where
        R: Read;
}
