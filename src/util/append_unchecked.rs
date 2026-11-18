use crate::Error;
use crate::Error::IntegerOverflow;
use std::mem::MaybeUninit;

/// The implementation of the `append_to_vec` functions that does not fill the new tail with zeros first.
///
/// Reserves the length from `len_fn` in the `target`, fills the reserved tail with `fill_fn`, and only then grows
/// the `target` over the tail. The `target` keeps its original length if `fill_fn` fails or panics.
///
/// The `fill_fn` is given the length from `len_fn` so it does not have to compute it again.
///
/// # Safety
/// The slice given to `fill_fn` is uninitialized memory of exactly the length from `len_fn`. The `fill_fn` must not
/// read from the slice, and it must write every byte of the slice before it returns `Ok`.
pub(crate) unsafe fn append_to_vec_with_unchecked<LF, FF>(
    target: &mut Vec<u8>,
    len_fn: LF,
    fill_fn: FF,
) -> Result<usize, Error>
where
    LF: FnOnce() -> Result<usize, Error>,
    FF: FnOnce(usize, &mut [u8]) -> Result<usize, Error>,
{
    let original_len: usize = target.len();
    let encoded_len: usize = len_fn()?;
    // The error is built here, since `ok_or` would build it and drop it on every call.
    let expanded_len: usize = match original_len.checked_add(encoded_len) {
        Some(expanded_len) => expanded_len,
        None => return Err(IntegerOverflow),
    };

    target.reserve(encoded_len);
    let tail: &mut [MaybeUninit<u8>] = &mut target.spare_capacity_mut()[..encoded_len];
    // SAFETY: A `MaybeUninit<u8>` has the layout of a `u8`, and the caller ensures `fill_fn` never reads the tail.
    // Miri accepts a `&mut [u8]` to uninitialized bytes that are never read, but the Rust reference has not settled
    // if such a reference is valid, and `-Zmiri-recursive-validation` rejects it.
    let tail: &mut [u8] = unsafe { &mut *(tail as *mut [MaybeUninit<u8>] as *mut [u8]) };
    let also_encoded_len: usize = fill_fn(encoded_len, tail)?;
    debug_assert_eq!(encoded_len, also_encoded_len);

    // SAFETY: The tail is within the reserved capacity, and the caller ensures `fill_fn` wrote every byte of it.
    unsafe { target.set_len(expanded_len) };
    Ok(encoded_len)
}

#[cfg(test)]
mod tests {
    use crate::Error;
    use crate::append_to_vec_with_unchecked;
    use std::panic::{AssertUnwindSafe, catch_unwind};

    /// Appends the `data` to the `target` with `append_to_vec_with_unchecked`.
    fn append(target: &mut Vec<u8>, data: &[u8]) -> Result<usize, Error> {
        unsafe {
            append_to_vec_with_unchecked(
                target,
                || Ok(data.len()),
                |len, tail| {
                    tail.copy_from_slice(data);
                    Ok(len)
                },
            )
        }
    }

    #[test]
    fn fn_append_to_vec_with_unchecked() {
        let test_cases: &[(&[u8], &[u8], &[u8])] = &[
            (b"", b"", b""),
            (b"", b"data", b"data"),
            (b"prefix", b"", b"prefix"),
            (b"prefix", b"data", b"prefixdata"),
        ];

        for (prefix, data, expected) in test_cases {
            let mut target: Vec<u8> = prefix.to_vec();
            let result: Result<usize, Error> = append(&mut target, data);
            assert!(
                matches!(result, Ok(len) if len == data.len()),
                "prefix={prefix:?} data={data:?}"
            );
            assert_eq!(
                target.as_slice(),
                *expected,
                "prefix={prefix:?} data={data:?}"
            );

            let result: Result<usize, Error> = append(&mut target, data);
            assert!(
                matches!(result, Ok(len) if len == data.len()),
                "prefix={prefix:?} data={data:?}"
            );
            assert_eq!(
                target.as_slice(),
                [*expected, *data].concat().as_slice(),
                "prefix={prefix:?} data={data:?}"
            );
        }
    }

    #[test]
    fn len_fn_error() {
        let mut target: Vec<u8> = b"prefix".to_vec();
        let result: Result<usize, Error> = unsafe {
            append_to_vec_with_unchecked(
                &mut target,
                || Err(Error::invalid_encoded_data()),
                |_, _| panic!("the fill function was called"),
            )
        };
        assert!(matches!(result, Err(Error::InvalidEncodedData { .. })));
        assert_eq!(target.as_slice(), b"prefix");
    }

    #[test]
    fn len_fn_overflow() {
        let mut target: Vec<u8> = b"prefix".to_vec();
        let result: Result<usize, Error> = unsafe {
            append_to_vec_with_unchecked(
                &mut target,
                || Ok(usize::MAX),
                |_, _| panic!("the fill function was called"),
            )
        };
        assert!(matches!(result, Err(Error::IntegerOverflow)));
        assert_eq!(target.as_slice(), b"prefix");
    }

    #[test]
    fn fill_fn_error() {
        let mut target: Vec<u8> = b"prefix".to_vec();
        let result: Result<usize, Error> = unsafe {
            append_to_vec_with_unchecked(
                &mut target,
                || Ok(4),
                |_, tail| {
                    tail[0] = 0xFF;
                    Err(Error::invalid_encoded_data())
                },
            )
        };
        assert!(matches!(result, Err(Error::InvalidEncodedData { .. })));
        assert_eq!(target.as_slice(), b"prefix");
    }

    #[test]
    fn fill_fn_panic() {
        let mut target: Vec<u8> = b"prefix".to_vec();
        let result: std::thread::Result<Result<usize, Error>> =
            catch_unwind(AssertUnwindSafe(|| unsafe {
                append_to_vec_with_unchecked(
                    &mut target,
                    || Ok(4),
                    |_, tail| {
                        tail[0] = 0xFF;
                        panic!("the fill function panicked")
                    },
                )
            }));
        assert!(result.is_err());
        assert_eq!(target.as_slice(), b"prefix");
    }
}
