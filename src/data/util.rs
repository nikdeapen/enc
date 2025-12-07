use crate::Error::IntegerOverflow;
use crate::{Encoder, Error};

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

    // todo -- this may expose uninitialized memory if `slice_fn` panics, this is also different
    // todo -- than the unsafe code in `EncodeToSlice` and it may be better to use the same code
    unsafe {
        target.reserve(encoded_len);
        let slice: *mut u8 = target.as_mut_ptr().add(original_len);
        let slice: &mut [u8] = std::slice::from_raw_parts_mut(slice, encoded_len);

        match slice_fn(data, slice) {
            Ok(also_encoded_len) => {
                debug_assert_eq!(encoded_len, also_encoded_len);
                target.set_len(expanded_len);
                Ok(encoded_len)
            }
            Err(error) => Err(error),
        }
    }
}

/// Appends the encoded `data` to the `target` string.
///
/// Returns the length of the encoded `data`.
///
/// # Safety
/// The encoded `data` must be a valid UTF-8 byte sequence.
#[allow(dead_code)]
pub(crate) unsafe fn append_to_string_unchecked<E>(
    encoder: &E,
    data: &[u8],
    target: &mut String,
) -> Result<usize, Error>
where
    E: Encoder,
{
    let encoded_len: usize = encoder.append_to_vec(data, unsafe { target.as_mut_vec() })?;
    debug_assert!(std::str::from_utf8(&target.as_bytes()[..(target.len() - encoded_len)]).is_ok());
    Ok(encoded_len)
}
