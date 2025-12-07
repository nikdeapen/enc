use crate::Error::{InsufficientTargetSpace, IntegerOverflow};
use crate::{EncodedLen, Error};

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
    /// This comes with two requirements:
    ///  1. The clients must ensure the `target` buffer has sufficient space as defined by the
    ///     `EncodedLen::encoded_len` implementation.
    ///  2. Implementations must ensure the length above matches the actual number of bytes written
    ///     to the `target` buffer when encoding the value.
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
    fn append_to_vec(&self, target: &mut Vec<u8>) -> Result<usize, Error> {
        let original_len: usize = target.len();
        let encoded_len: usize = self.encoded_len()?;
        let expanded_len: usize = original_len
            .checked_add(encoded_len)
            .ok_or(IntegerOverflow)?;

        // todo -- this may expose uninitialized memory if `encode_to_slice_unchecked` panics
        #[allow(clippy::uninit_vec)]
        unsafe {
            target.reserve(encoded_len);
            target.set_len(expanded_len);
            let t: &mut [u8] = &mut target[original_len..];
            match self.encode_to_slice_unchecked(t) {
                Ok(also_encoded_len) => {
                    debug_assert_eq!(encoded_len, also_encoded_len);
                    Ok(encoded_len)
                }
                Err(error) => {
                    target.set_len(original_len);
                    Err(error)
                }
            }
        }
    }

    /// Encodes the value as a vec. Returns the vec.
    fn encode_as_vec(&self) -> Result<Vec<u8>, Error> {
        let mut vec: Vec<u8> = Vec::default();
        self.append_to_vec(&mut vec)?;
        Ok(vec)
    }
}
