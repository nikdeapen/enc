use std::io;

use crate::value::decode_from_read::DecodeFromRead;
use crate::value::util::{read_optional_byte, read_single_byte};
use crate::var_int::VarIntSize;
use crate::{DecodeFromReadPrefix, ReadLimit};

/// A value that can decode itself from a `Read` where the first bytes from the `Read` are a
/// variable length encoded length prefix.
///
/// Notes:
/// - The decoder does not need to fully drain the `Read`.
/// - The decoder must not read past the end of the encoded value which is denoted by the prefix.
pub trait DecodeFromReadLengthPrefixed: DecodeFromRead {
    /// Decodes a length-prefixed value from the `Read`.
    fn decode_from_read_length_prefixed<R>(r: &mut R) -> Result<Self, io::Error>
    where
        R: io::Read,
    {
        let first: u8 = read_single_byte(r)?;
        Self::decode_from_read_length_prefixed_with_first_byte(first, r)
    }

    /// Decodes a length-prefixed value from the `Read` prefix given the first byte.
    fn decode_from_read_length_prefixed_with_first_byte<R>(
        first: u8,
        r: &mut R,
    ) -> Result<Self, io::Error>
    where
        R: io::Read,
    {
        let len: usize = VarIntSize::decode_from_read_prefix_with_first_byte(first, r)?.value;
        let mut r: ReadLimit<R> = ReadLimit {
            read: r,
            limit: len,
        };
        Self::decode_from_read(&mut r)
    }

    /// Decodes an optional value from the `Read` prefix. Returns `None` if the `Read` is empty.
    fn decode_from_read_prefix_optional<R>(r: &mut R) -> Result<Option<Self>, io::Error>
    where
        R: io::Read,
    {
        if let Some(first) = read_optional_byte(r)? {
            Ok(Some(
                Self::decode_from_read_length_prefixed_with_first_byte(first, r)?,
            ))
        } else {
            Ok(None)
        }
    }
}
