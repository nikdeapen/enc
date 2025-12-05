use crate::{data, Error};

/// Responsible for decoding binary data.
pub trait Decoder {
    /// Gets the length of the decoded `data`.
    fn decoded_len(&self, data: &[u8]) -> Result<usize, Error>;

    /// Encodes the `data` into the `target` slice.
    ///
    /// Returns the length of the decoded `data`.
    fn decode_to_slice(&self, data: &[u8], target: &mut [u8]) -> Result<usize, Error>;

    /// Appends the decoded `data` to the `target` vec.
    ///
    /// Returns the length of the decoded `data`.
    fn append_to_vec(&self, data: &[u8], target: &mut Vec<u8>) -> Result<usize, Error> {
        data::util::default_append_to_vec(
            data,
            target,
            |data| self.decoded_len(data),
            |data, target| self.decode_to_slice(data, target),
        )
    }

    /// Encodes the `data` as a vec.
    ///
    /// Returns the vec.
    fn decode_as_vec(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        let mut vec: Vec<u8> = Vec::default();
        self.append_to_vec(data, &mut vec)?;
        Ok(vec)
    }
}
