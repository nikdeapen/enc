use crate::Error;
use crate::Error::IntegerOverflow;

/// Responsible for encoding binary data.
pub trait Encoder {
    /// Gets the length of the encoded `data`.
    fn encoded_len(&self, data: &[u8]) -> Result<usize, Error>;

    /// Encodes the `data` into the `target` slice.
    ///
    /// Returns the length of the encoded `data`.
    fn encode_to_slice(&self, data: &[u8], target: &mut [u8]) -> Result<usize, Error>;

    /// Appends the encoded `data` to the `target` vec.
    ///
    /// Returns the length of the encoded `data`.
    fn append_to_vec(&self, data: &[u8], target: &mut Vec<u8>) -> Result<usize, Error> {
        let original_len: usize = target.len();
        let encoded_len: usize = self.encoded_len(data)?;
        let expanded_len: usize = original_len
            .checked_add(encoded_len)
            .ok_or(IntegerOverflow)?;
        target.resize(expanded_len, 0u8);
        let slice: &mut [u8] = &mut target.as_mut_slice()[original_len..];
        match self.encode_to_slice(data, slice) {
            Ok(also_encoded_len) => {
                debug_assert_eq!(encoded_len, also_encoded_len);
                Ok(encoded_len)
            }
            Err(error) => {
                target.truncate(original_len);
                Err(error)
            }
        }
    }

    /// Encodes the `data` as a vec.
    ///
    /// Returns the vec.
    fn encode_as_vec(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        let mut vec: Vec<u8> = Vec::default();
        self.append_to_vec(data, &mut vec)?;
        Ok(vec)
    }
}
