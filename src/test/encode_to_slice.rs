use crate::test::hex;
use crate::EncodeToSlice;
use std::fmt::Debug;

/// Tests the `EncodeToSlice` trait.
pub fn test_encode_to_slice<T>(value: &T, expected: &[u8])
where
    T: EncodeToSlice + Debug,
{
    let mut buffer: Vec<u8> = vec![0u8; expected.len() + 1];
    match value.encode_to_slice(&mut buffer) {
        Ok(encoded_len) => {
            assert_eq!(encoded_len, expected.len(), "value={:?}", value);
            assert_eq!(buffer[expected.len()], 0, "value={:?}", value);

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
            panic!("value={:?} error={:?}", value, error);
        }
    }
}
