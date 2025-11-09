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
        default_append_to_vec(
            data,
            target,
            |data| self.encoded_len(data),
            |data, target| self.encode_to_slice(data, target),
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

/// The default implementation of the `append_to_vec` function for `Encoder`s and `Decoder`s.
///
/// For `len_fn` see `Encoder::encoded_len` and `Decoder::decoded_len`.
/// For `slice_fn` see `Encoder::encode_to_slice` and `Decoder::decode_to_slice`.
pub(crate) fn default_append_to_vec<LF, SF>(
    data: &[u8],
    target: &mut Vec<u8>,
    len_fn: LF,
    slice_fn: SF,
) -> Result<usize, Error>
where
    LF: Fn(&[u8]) -> Result<usize, Error>,
    SF: Fn(&[u8], &mut [u8]) -> Result<usize, Error>,
{
    let original_len: usize = target.len();
    let encoded_len: usize = len_fn(data)?;
    let expanded_len: usize = original_len
        .checked_add(encoded_len)
        .ok_or(IntegerOverflow)?;

    target.reserve(encoded_len);
    let slice: *mut u8 = unsafe { target.as_mut_ptr().add(original_len) };
    let slice: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(slice, encoded_len) };

    match slice_fn(data, slice) {
        Ok(also_encoded_len) => {
            debug_assert_eq!(encoded_len, also_encoded_len);
            unsafe { target.set_len(expanded_len) };
            Ok(encoded_len)
        }
        Err(error) => Err(error),
    }
}
