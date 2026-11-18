use crate::test::hex;
use crate::{EncodeToSlice, EncodeToWrite, EncodedLen, Error};
use std::fmt::Debug;
use std::io::Cursor;

/// Tests the encoding traits.
pub fn test_encode<T>(value: &T, expected: &[u8])
where
    T: EncodedLen + EncodeToSlice + EncodeToWrite + Debug,
{
    test_encoded_len(value, expected.len());
    test_encode_to_slice(value, expected);
    test_append_to_vec(value, expected);
    test_encode_to_write(value, expected);
}

/// Tests the `EncodedLen` trait.
pub fn test_encoded_len<T>(value: &T, expected: usize)
where
    T: EncodedLen + Debug,
{
    match value.encoded_len() {
        Ok(encoded_len) => {
            assert_eq!(encoded_len, expected, "value={value:?}");
        }
        Err(error) => {
            panic!("value={value:?} error={error:?}");
        }
    }
}

/// Tests the `EncodeToSlice` trait.
pub fn test_encode_to_slice<T>(value: &T, expected: &[u8])
where
    T: EncodeToSlice + Debug,
{
    let mut buffer: Vec<u8> = vec![0u8; expected.len() + 1];
    match value.encode_to_slice(&mut buffer) {
        Ok(encoded_len) => {
            assert_eq!(encoded_len, expected.len(), "value={value:?}");
            assert_eq!(buffer[expected.len()], 0, "value={value:?}");

            let encoded: &[u8] = &buffer[..expected.len()];
            assert_eq!(
                encoded,
                expected,
                "value={value:?} encoded={} expected={}",
                hex(encoded),
                hex(expected)
            );
        }
        Err(error) => {
            panic!("value={value:?} error={error:?}");
        }
    }

    let mut buffer: Vec<u8> = vec![0u8; expected.len()];
    match value.encode_to_slice(&mut buffer) {
        Ok(encoded_len) => {
            assert_eq!(encoded_len, expected.len(), "value={value:?}");
            assert_eq!(
                buffer.as_slice(),
                expected,
                "value={value:?} encoded={} expected={}",
                hex(buffer.as_slice()),
                hex(expected)
            );
        }
        Err(error) => {
            panic!("value={value:?} error={error:?}");
        }
    }

    if !expected.is_empty() {
        let mut buffer: Vec<u8> = vec![0u8; expected.len() - 1];
        match value.encode_to_slice(&mut buffer) {
            Ok(encoded_len) => panic!("value={value:?} encoded_len={encoded_len}"),
            Err(Error::InsufficientTargetSpace) => {}
            Err(error) => panic!("value={value:?} error={error:?}"),
        }
    }
}

/// Tests the `append_to_vec` & `encode_as_vec` functions of the `EncodeToSlice` trait.
///
/// # Note
/// - The value is appended to a vec whose spare capacity holds guard bytes, so a byte that the encoding skips
///   shows up as a guard byte.
pub fn test_append_to_vec<T>(value: &T, expected: &[u8])
where
    T: EncodeToSlice + Debug,
{
    const GUARD: u8 = 0xA5;
    const PREFIX_LEN: usize = 2;

    let mut buffer: Vec<u8> = Vec::default();
    match value.append_to_vec(&mut buffer) {
        Ok(encoded_len) => {
            assert_eq!(encoded_len, expected.len(), "value={value:?}");
            assert_eq!(
                buffer.as_slice(),
                expected,
                "value={value:?} encoded={} expected={}",
                hex(buffer.as_slice()),
                hex(expected)
            );
        }
        Err(error) => {
            panic!("value={value:?} error={error:?}");
        }
    }

    let mut buffer: Vec<u8> = vec![GUARD; PREFIX_LEN + expected.len() + 1];
    buffer.truncate(PREFIX_LEN);
    match value.append_to_vec(&mut buffer) {
        Ok(encoded_len) => {
            assert_eq!(encoded_len, expected.len(), "value={value:?}");
            assert_eq!(
                &buffer[..PREFIX_LEN],
                &[GUARD; PREFIX_LEN],
                "value={value:?}"
            );

            let encoded: &[u8] = &buffer[PREFIX_LEN..];
            assert_eq!(
                encoded,
                expected,
                "value={value:?} encoded={} expected={}",
                hex(encoded),
                hex(expected)
            );
        }
        Err(error) => {
            panic!("value={value:?} error={error:?}");
        }
    }

    match value.encode_as_vec() {
        Ok(encoded) => {
            assert_eq!(
                encoded.as_slice(),
                expected,
                "value={value:?} encoded={} expected={}",
                hex(encoded.as_slice()),
                hex(expected)
            );
        }
        Err(error) => {
            panic!("value={value:?} error={error:?}");
        }
    }
}

/// Tests the `EncodeToWrite` trait.
pub fn test_encode_to_write<T>(value: &T, expected: &[u8])
where
    T: EncodeToWrite + Debug,
{
    let mut w: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    match value.encode_to_write(&mut w) {
        Ok(encoded_len) => {
            assert_eq!(encoded_len, expected.len(), "value={value:?}");

            let buffer: Vec<u8> = w.into_inner();
            assert_eq!(buffer.len(), expected.len(), "value={value:?}");

            let encoded: &[u8] = &buffer[..expected.len()];
            assert_eq!(
                encoded,
                expected,
                "value={value:?} encoded={} expected={}",
                hex(encoded),
                hex(expected)
            );
        }
        Err(error) => {
            panic!("value={value:?} error={error:?}");
        }
    }
}
