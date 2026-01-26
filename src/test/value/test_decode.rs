use crate::test::hex;
use crate::var_int::VarIntSize;
use crate::{read_optional_byte, DecodeFromRead, DecodeFromReadPrefix, EncodeToWrite};
use std::fmt::Debug;
use std::io::{Cursor, Write};

/// Tests the decoding traits.
pub fn test_decode<T>(encoded: &[u8], expected: &T, add_len_prefix: bool)
where
    T: DecodeFromRead + DecodeFromReadPrefix + PartialEq + Debug,
{
    test_decode_from_read(encoded, expected);
    test_decode_from_read_prefix(encoded, expected, add_len_prefix);
}

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
