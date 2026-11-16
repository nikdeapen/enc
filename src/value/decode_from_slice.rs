use crate::Error;

/// A value that can decode itself from a slice.
pub trait DecodeFromSlice: Sized {
    /// Decodes a value from the `data` slice.
    ///
    /// # Note
    /// - The `data` slice must contain only the encoded value.
    fn decode_from_slice(data: &[u8]) -> Result<Self, Error>;
}
