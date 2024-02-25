use crate::EncodedLen;
use crate::Error::InsufficientTargetSpace;

/// A value that can encode itself to a slice.
pub trait EncodeToSlice: EncodedLen {
    /// Encodes the value to the target slice. Returns the length of the encoded value.
    ///
    /// # Unsafe
    /// This function is unsafe so implementations can assume the target slice has sufficient space
    /// for the encoded value. Clients must ensure this invariant always holds true.
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> usize;

    /// Encodes the value to the target slice. Returns the length of the encoded value.
    fn encode_to_slice(&self, target: &mut [u8]) -> Result<usize, crate::Error> {
        let encoded_len: usize = self.encoded_len();
        if encoded_len > target.len() {
            Err(InsufficientTargetSpace)
        } else {
            let also_encoded_len: usize = unsafe { self.encode_to_slice_unchecked(target) };
            debug_assert_eq!(encoded_len, also_encoded_len);
            Ok(encoded_len)
        }
    }

    /// Appends the encoded value to the target vec. Returns the length of the encoded value.
    fn append_to_vec(&self, target: &mut Vec<u8>) -> usize {
        let original_len: usize = target.len();
        let encoded_len: usize = self.encoded_len();
        target.resize(original_len + encoded_len, 0u8);
        let also_encoded_len: usize =
            unsafe { self.encode_to_slice_unchecked(&mut target.as_mut_slice()[original_len..]) };
        debug_assert_eq!(encoded_len, also_encoded_len);
        encoded_len
    }

    /// Encodes the value as a vec. Returns the vec.
    fn encode_as_vec(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::default();
        self.append_to_vec(&mut vec);
        vec
    }
}
