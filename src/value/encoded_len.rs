use crate::Error;

/// A value with an encoded length.
///
/// # Safety
/// `EncodeToSlice` callers size their buffers from `encoded_len`, and `EncodeToSlice::encode_to_slice_unchecked` is
/// allowed to fill that many bytes without bounds checks. A length shorter than the value actually encodes to lets
/// the encoding write out of bounds, so this trait is `unsafe` to implement even though the function is safe to call.
///
/// Implementations must return the exact encoded length, and must return the same length for an unchanged value.
pub unsafe trait EncodedLen {
    /// Gets the length of the encoded value.
    fn encoded_len(&self) -> Result<usize, Error>;
}
