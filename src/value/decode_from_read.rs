use std::io::{Error, Read};

use crate::var_int::VarIntSize;
use crate::{read_single_byte, DecodeFromReadPrefix, ExactRead};

/// A value that can decode itself from a `Read`.
///
/// Note:
/// - The decoder must fully drain the `Read`.
pub trait DecodeFromRead: Sized {
    /// Decodes a value from the `Read`.
    fn decode_from_read<R>(r: &mut R) -> Result<Self, Error>
    where
        R: Read;

    /// Decodes a length-prefixed value from the `Read` prefix.
    #[cfg(feature = "var-int")]
    fn decode_from_read_length_prefixed<R>(r: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        Self::decode_from_read_length_prefixed_with_first_byte(read_single_byte(r)?, r)
    }

    /// Decodes a length-prefixed value from the `Read` prefix given the first byte.
    #[cfg(feature = "var-int")]
    fn decode_from_read_length_prefixed_with_first_byte<R>(
        first: u8,
        r: &mut R,
    ) -> Result<Self, Error>
    where
        R: Read,
    {
        let len_prefix: usize =
            VarIntSize::decode_from_read_prefix_with_first_byte(first, r)?.value;
        let mut exact: ExactRead<R> = ExactRead::new(r, len_prefix);
        Self::decode_from_read(&mut exact)
    }
}
