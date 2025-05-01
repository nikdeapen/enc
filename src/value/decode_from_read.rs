use std::io::{Error, Read};

/// A value that can decode itself from a `Read`.
pub trait DecodeFromRead: Sized {
    /// Decodes a value from the `Read`.
    ///
    /// # Note
    /// The impl must fully drain the `Read`.
    fn decode_from_read<R>(r: &mut R) -> Result<Self, Error>
    where
        R: Read;

    /// Decodes a length-prefixed value from the `Read` prefix.
    #[cfg(feature = "var-int")]
    fn decode_from_read_length_prefixed<R>(r: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        Self::decode_from_read_length_prefixed_with_first_byte(crate::read_single_byte(r)?, r)
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
        use crate::DecodeFromReadPrefix;
        let len_prefix: usize =
            crate::var_int::VarIntSize::decode_from_read_prefix_with_first_byte(first, r)?.value;
        Self::decode_from_read(&mut r.take(len_prefix as u64))
    }
}
