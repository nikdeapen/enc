use crate::test::hex;
use crate::{read_optional_byte, DecodeFromRead};
use std::fmt::Debug;
use std::io::Cursor;

/// Tests the `DecodeFromRead` trait.
pub fn test_decode_from_read<T>(encoded: &[u8], expected: &T)
where
    T: DecodeFromRead + PartialEq + Debug,
{
    let mut r: Cursor<Vec<u8>> = Cursor::new(encoded.into());

    match T::decode_from_read(&mut r) {
        Ok(decoded) => {
            assert_eq!(&decoded, expected, "encoded={}", hex(encoded));
            assert_eq!(
                read_optional_byte(&mut r).unwrap(),
                None,
                "encoded={}",
                hex(encoded)
            );
        }
        Err(error) => {
            panic!(
                "encoded={} expected={:?}, error={:?}",
                hex(encoded),
                expected,
                error
            )
        }
    }
}
