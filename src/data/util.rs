#[cfg(any(feature = "hex", feature = "base-64", feature = "percent"))]
use crate::Encoder;
#[cfg(any(feature = "hex", feature = "base-64", feature = "percent"))]
use crate::Error;

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
