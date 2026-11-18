use crate::Error::InsufficientTargetSpace;
use crate::{EncodedLen, Error, append_to_vec_with_unchecked};

/// A value that can encode itself to a slice.
pub trait EncodeToSlice: EncodedLen {
    /// Encodes the value to the `target` slice.
    ///
    /// Returns the length of the encoded value.
    ///
    /// # Note
    /// - The impl must not read from the `target` slice.
    /// - The impl must not write to the `target` slice outside the range: [0, encoded_len - 1].
    /// - The impl must overwrite the entire range: [0, encoded_len - 1].
    ///
    /// # Safety
    /// This function is `unsafe` so implementations can assume the `target` slice has sufficient
    /// space for the encoded value. This allows implementations to avoid computing an encoded
    /// length when determining a buffer size and recomputing it when encoding the value.
    ///
    /// This comes with three requirements:
    ///  1. The clients must ensure the `target` buffer has sufficient space as defined by the
    ///     `EncodedLen::encoded_len` implementation.
    ///  2. Implementations must ensure the length above matches the actual number of bytes written
    ///     to the `target` buffer when encoding the value.
    ///  3. Implementations must follow the notes above. The `append_to_vec` function gives the `target` buffer as
    ///     uninitialized memory, so reading from it or leaving a byte of the range unwritten is undefined behavior.
    ///
    /// `EncodedLen` is an `unsafe trait` so the length in requirement 1 can be trusted without checking it, and so
    /// its implementations answer for requirements 2 and 3.
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error>;

    /// Encodes the value to the `target` slice.
    ///
    /// Returns the length of the encoded value.
    ///
    /// See: `encode_to_slice_unchecked`.
    fn encode_to_slice(&self, target: &mut [u8]) -> Result<usize, Error> {
        let encoded_len: usize = self.encoded_len()?;
        if encoded_len > target.len() {
            Err(InsufficientTargetSpace)
        } else {
            let also_encoded_len: usize = unsafe { self.encode_to_slice_unchecked(target)? };
            debug_assert_eq!(encoded_len, also_encoded_len);
            Ok(encoded_len)
        }
    }

    /// Appends the encoded value to the `target` vec.
    ///
    /// Returns the length of the encoded value.
    ///
    /// # Note
    /// - The value is encoded into the spare capacity of the `target` vec, which is not filled with zeros first.
    fn append_to_vec(&self, target: &mut Vec<u8>) -> Result<usize, Error> {
        // SAFETY: The `encode_to_slice_unchecked` function is given exactly `encoded_len` bytes. It never reads from
        // them and it writes every one of them, which the `unsafe impl` of `EncodedLen` answers for.
        unsafe {
            append_to_vec_with_unchecked(
                target,
                || self.encoded_len(),
                |_, t| self.encode_to_slice_unchecked(t),
            )
        }
    }

    /// Encodes the value as a vec. Returns the vec.
    fn encode_as_vec(&self) -> Result<Vec<u8>, Error> {
        let encoded_len: usize = self.encoded_len()?;
        // The exact capacity skips the grow path of the vec, which costs more than encoding a small value does.
        let mut vec: Vec<u8> = Vec::with_capacity(encoded_len);
        // SAFETY: The same as `append_to_vec`.
        unsafe {
            append_to_vec_with_unchecked(
                &mut vec,
                || Ok(encoded_len),
                |_, t| self.encode_to_slice_unchecked(t),
            )?;
        }
        Ok(vec)
    }
}
