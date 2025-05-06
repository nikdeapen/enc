use crate::test::hex;
use crate::{read_optional_byte, DecodeFromReadPrefix};
use std::fmt::Debug;
use std::io::Cursor;

/// Tests the `DecodeFromReadPrefix` trait.
pub fn test_decode_from_read_prefix<T>(encoded: &[u8], expected: T)
where
    T: DecodeFromReadPrefix + PartialEq + Debug,
{
    let mut buffer: Vec<u8> = encoded.into();
    buffer.push(0u8);
    let mut r: Cursor<Vec<u8>> = Cursor::new(buffer);

    match T::decode_from_read_prefix(&mut r) {
        Ok(decoded) => {
            assert_eq!(decoded, expected, "encoded={}", hex(encoded));
            assert_eq!(
                read_optional_byte(&mut r).unwrap(),
                Some(0),
                "encoded={}",
                hex(encoded)
            );
            assert_eq!(
                read_optional_byte(&mut r).unwrap(),
                None,
                "encoded={}",
                hex(encoded)
            );
        }
        Err(error) => {
            assert!(
                false,
                "encoded={} expected={:?}, error={:?}",
                hex(encoded),
                expected,
                error
            )
        }
    }
}
