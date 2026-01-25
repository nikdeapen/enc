use crate::{data, Error};

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
        data::util::default_append_to_vec(
            data,
            target,
            |d| self.encoded_len(d),
            |d, t| self.encode_to_slice(d, t),
        )
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
