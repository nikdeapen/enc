use std::io;

/// A value that can decode itself from a `Read`.
///
/// Note:
/// - The encoded value will be encoded with all the bytes from the `Read`.
/// - The decoder should drain the `Read` to decode the full value.
pub trait DecodeFromRead: Sized {
    /// Decodes a value from the `Read`.
    fn decode_from_read<R>(r: &mut R) -> Result<Self, io::Error>
    where
        R: io::Read;
}
