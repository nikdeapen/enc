use crate::Error;
use std::io::Read;

/// A value that can decode itself from a `Read`.
pub trait DecodeFromRead: Sized {
    /// Decodes a value from the `Read`.
    ///
    /// # Note
    /// The implementation must fully drain the `Read`.
    fn decode_from_read<R>(r: &mut R) -> Result<Self, Error>
    where
        R: Read;

    /// Decodes a length-prefixed value from the `Read` prefix.
    #[cfg(feature = "var-int")]
    fn decode_from_read_length_prefixed<R>(r: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let first: u8 = crate::read_single_byte(r)?;
        Self::decode_from_read_length_prefixed_with_first_byte(r, first)
    }

    /// Decodes a length-prefixed value from the `Read` prefix given the `first` byte.
    #[cfg(feature = "var-int")]
    fn decode_from_read_length_prefixed_with_first_byte<R>(
        r: &mut R,
        first: u8,
    ) -> Result<Self, Error>
    where
        R: Read,
    {
        use crate::var_int::VarIntSize;
        use crate::DecodeFromReadPrefix;

        let prefix: usize = VarIntSize::decode_from_read_prefix_with_first_byte(r, first)?.value();
        const _: () = assert!(usize::BITS <= 64);
        Self::decode_from_read(&mut r.take(prefix as u64))
    }
}
