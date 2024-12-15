use std::io::{Error, Read};

/// A value that can decode itself from a `Read`.
///
/// Note:
/// - The decoder must fully drain the `Read`.
pub trait DecodeFromRead: Sized {
    /// Decodes a value from the `Read`.
    fn decode_from_read<R>(r: &mut R) -> Result<Self, Error>
    where
        R: Read;
}
