use crate::Error::IntegerOverflow;
use crate::{Encoder, Error};

/// The default implementation of the `append_to_vec` function for `Encoder`s and `Decoder`s.
///
/// For `len_fn` see `Encoder::encoded_len` and `Decoder::decoded_len`.
/// For `code_fn` see `Encoder::encode_to_slice` and `Decoder::decode_to_slice`.
pub(crate) fn default_append_to_vec<LF, CF>(
    data: &[u8],
    target: &mut Vec<u8>,
    len_fn: LF,
    code_fn: CF,
) -> Result<usize, Error>
where
    LF: Fn(&[u8]) -> Result<usize, Error>,
    CF: Fn(&[u8], &mut [u8]) -> Result<usize, Error>,
{
    let original_len: usize = target.len();
    let encoded_len: usize = len_fn(data)?;
    let expanded_len: usize = original_len
        .checked_add(encoded_len)
        .ok_or(IntegerOverflow)?;

    target.resize(expanded_len, 0u8);
    match code_fn(data, &mut target[original_len..]) {
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

/// Appends the encoded `data` to the `target` string.
///
/// Returns the length of the encoded `data`.
///
/// # Safety
/// The encoded `data` must be a valid UTF-8 byte sequence.
#[cfg(any(feature = "hex", feature = "base-64", feature = "percent"))]
pub(crate) unsafe fn append_to_string_unchecked<E>(
    encoder: &E,
    data: &[u8],
    target: &mut String,
) -> Result<usize, Error>
where
    E: Encoder,
{
    let encoded_len: usize = encoder.append_to_vec(data, unsafe { target.as_mut_vec() })?;
    debug_assert!(std::str::from_utf8(&target.as_bytes()[(target.len() - encoded_len)..]).is_ok());
    Ok(encoded_len)
}
