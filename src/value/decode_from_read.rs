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

    /// Decodes a length-prefixed value from the `Read`.
    #[cfg(feature = "var-int")]
    fn decode_from_read_length_prefixed<R>(r: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        Self::decode_from_read_length_prefixed_with_first_byte(crate::util::read_single_byte(r)?, r)
    }

    /// Decodes a length-prefixed value from the `Read` given the `first` byte from the `Read`.
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
        let mut r: Box<dyn Read> = Box::new(crate::ExactRead::new(r, len_prefix));
        let result: Self = Self::decode_from_read(&mut r)?;
        Ok(result)
    }
}
