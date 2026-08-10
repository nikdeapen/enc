use crate::{Error, append_to_vec_with};

/// Responsible for decoding binary data.
pub trait Decoder {
    /// Gets the length of the decoded `data`.
    ///
    /// The result must exactly equal the number of bytes `decode_to_slice` will write for the same `data`.
    fn decoded_len(&self, data: &[u8]) -> Result<usize, Error>;

    /// Decodes the `data` into the `target` slice.
    ///
    /// Returns the length of the decoded `data`.
    fn decode_to_slice(&self, data: &[u8], target: &mut [u8]) -> Result<usize, Error>;

    /// Appends the decoded `data` to the `target` vec.
    ///
    /// Returns the length of the decoded `data`.
    fn append_to_vec(&self, data: &[u8], target: &mut Vec<u8>) -> Result<usize, Error> {
        append_to_vec_with(
            target,
            || self.decoded_len(data),
            |t| self.decode_to_slice(data, t),
        )
    }

    /// Decodes the `data` as a vec.
    ///
    /// Returns the vec.
    fn decode_as_vec(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        let mut vec: Vec<u8> = Vec::default();
        self.append_to_vec(data, &mut vec)?;
        Ok(vec)
    }
}
