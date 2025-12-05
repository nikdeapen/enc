use crate::{read_optional_byte, read_single_byte, Error};
use std::io::Read;

/// A value that can decode itself from a `Read` prefix.
///
/// # Note
/// - The impl does not need to fully drain the `Read`.
/// - The impl must read the entire encoded value.
/// - The impl must not read past the end of the encoded value.
/// - An encoded value must be at least one byte in length.
pub trait DecodeFromReadPrefix: Sized {
    /// Decodes a value from the `Read` prefix.
    fn decode_from_read_prefix<R>(r: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let first: u8 = read_single_byte(r)?;
        Self::decode_from_read_prefix_with_first_byte(r, first)
    }

    /// Decodes a value from the `Read` prefix given the `first` byte.
    fn decode_from_read_prefix_with_first_byte<R>(r: &mut R, first: u8) -> Result<Self, Error>
    where
        R: Read;

    /// Decodes an optional value from the `Read` prefix.
    ///
    /// Returns `None` if the `Read` is empty.
    fn decode_from_read_prefix_optional<R>(r: &mut R) -> Result<Option<Self>, Error>
    where
        R: Read,
    {
        if let Some(first) = read_optional_byte(r)? {
            Ok(Some(Self::decode_from_read_prefix_with_first_byte(
                r, first,
            )?))
        } else {
            Ok(None)
        }
    }
}
