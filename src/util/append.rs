use crate::Error;
use crate::Error::IntegerOverflow;

/// The default implementation of the `append_to_vec` functions.
///
/// Grows the `target` by the length from `len_fn`, fills the new tail with `fill_fn`, and truncates the `target`
/// back to its original length if `fill_fn` fails.
pub(crate) fn append_to_vec_with<LF, FF>(
    target: &mut Vec<u8>,
    len_fn: LF,
    fill_fn: FF,
) -> Result<usize, Error>
where
    LF: FnOnce() -> Result<usize, Error>,
    FF: FnOnce(&mut [u8]) -> Result<usize, Error>,
{
    let original_len: usize = target.len();
    let encoded_len: usize = len_fn()?;
    let expanded_len: usize = original_len
        .checked_add(encoded_len)
        .ok_or(IntegerOverflow)?;

    target.resize(expanded_len, 0u8);
    match fill_fn(&mut target[original_len..]) {
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
