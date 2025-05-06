use crate::test::hex;
use crate::EncodeToWrite;
use std::fmt::Debug;
use std::io::Cursor;

/// Tests the `EncodeToWrite` trait.
pub fn test_encode_to_write<T>(value: &T, expected: &[u8])
where
    T: EncodeToWrite + Debug,
{
    let mut w: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    match value.encode_to_write(&mut w) {
        Ok(encoded_len) => {
            assert_eq!(encoded_len, expected.len(), "value={:?}", value);

            let buffer: Vec<u8> = w.into_inner();
            assert_eq!(buffer.len(), expected.len(), "value={:?}", value);

            let encoded: &[u8] = &buffer[..expected.len()];
            assert_eq!(
                encoded,
                expected,
                "value={:?} encoded={} expected={}",
                value,
                hex(encoded),
                hex(expected)
            );
        }
        Err(error) => {
            assert!(false, "value={:?} error={:?}", value, error);
        }
    }
}
