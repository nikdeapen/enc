use crate::test::hex;
use crate::var_int::VarIntSize;
use crate::{read_optional_byte, DecodeFromReadPrefix, EncodeToWrite};
use std::fmt::Debug;
use std::io::{Cursor, Write};

/// Tests the `DecodeFromReadPrefix` trait.
pub fn test_decode_from_read_prefix<T>(encoded: &[u8], expected: &T, add_len_prefix: bool)
where
    T: DecodeFromReadPrefix + PartialEq + Debug,
{
    let buffer: Vec<u8> = Vec::default();
    let mut w: Cursor<Vec<u8>> = Cursor::new(buffer);
    if add_len_prefix {
        VarIntSize::from(encoded.len())
            .encode_to_write(&mut w)
            .unwrap();
    }
    w.write_all(encoded).unwrap();
    w.write_all(&[0]).unwrap();
    let mut r: Cursor<Vec<u8>> = Cursor::new(w.into_inner());
    match T::decode_from_read_prefix(&mut r) {
        Ok(decoded) => {
            assert_eq!(&decoded, expected, "encoded={}", hex(encoded));
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
            panic!(
                "encoded={} expected={:?}, error={:?}",
                hex(encoded),
                expected,
                error
            )
        }
    }
}
