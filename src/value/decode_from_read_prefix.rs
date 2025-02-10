use std::io;

use crate::{read_optional_byte, read_single_byte};

/// A value that can decode itself from a `Read` prefix.
///
/// # Note
/// - The decoder does not need to fully drain the `Read`.
/// - The decoder must read the entire encoded value.
/// - The decoder must not read past the end of the encoded value.
/// - An encoded value must be at least one byte in length.
pub trait DecodeFromReadPrefix: Sized {
    /// Decodes a value from the `Read` prefix.
    fn decode_from_read_prefix<R>(r: &mut R) -> Result<Self, io::Error>
    where
        R: io::Read,
    {
        let first: u8 = read_single_byte(r)?;
        Self::decode_from_read_prefix_with_first_byte(first, r)
    }

    /// Decodes a value from the `Read` prefix given the `first` byte.
    fn decode_from_read_prefix_with_first_byte<R>(first: u8, r: &mut R) -> Result<Self, io::Error>
    where
        R: io::Read;

    /// Decodes an optional value from the `Read` prefix.
    ///
    /// Returns `None` if the `Read` is empty.
    fn decode_from_read_prefix_optional<R>(r: &mut R) -> Result<Option<Self>, io::Error>
    where
        R: io::Read,
    {
        if let Some(first) = read_optional_byte(r)? {
            Ok(Some(Self::decode_from_read_prefix_with_first_byte(
                first, r,
            )?))
        } else {
            Ok(None)
        }
    }
}
