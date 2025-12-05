use crate::test::hex;
use crate::{EncodeToSlice, EncodeToWrite, EncodedLen};
use std::fmt::Debug;
use std::io::Cursor;

/// Tests the encoding traits.
pub fn test_encode<T>(value: &T, expected: &[u8])
where
    T: EncodedLen + EncodeToSlice + EncodeToWrite + Debug,
{
    test_encoded_len(value, expected.len());
    test_encode_to_slice(value, expected);
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
