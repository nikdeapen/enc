use crate::Error;
use crate::Error::IntegerOverflow;

/// Responsible for decoding binary data.
pub trait Decoder {
    /// Gets the length of the decoded data.
    fn decoded_len(&self, data: &[u8]) -> Result<usize, Error>;

    /// Decodes the data to the target slice. Returns the length of the decoded data.
    fn decode_to_slice(&self, data: &[u8], target: &mut [u8]) -> Result<usize, Error>;

    /// Appends the decoded data to the target vec. Returns the length of the decoded data.
    fn append_to_vec(&self, data: &[u8], target: &mut Vec<u8>) -> Result<usize, Error> {
        let original_len: usize = target.len();
        let decoded_len: usize = self.decoded_len(data)?;
        let expanded_len: usize = original_len
            .checked_add(decoded_len)
            .ok_or(IntegerOverflow)?;
        target.resize(expanded_len, 0u8);
        match self.decode_to_slice(data, &mut target.as_mut_slice()[original_len..]) {
            Ok(len) => {
                debug_assert_eq!(len, decoded_len);
                Ok(len)
            }
            Err(error) => {
                target.truncate(original_len);
                Err(error)
            }
        }
    }

    /// Decodes the data as a vec. Returns the vec.
    fn decode_as_vec(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        let mut vec: Vec<u8> = Vec::default();
        self.append_to_vec(data, &mut vec)?;
        Ok(vec)
    }
}
