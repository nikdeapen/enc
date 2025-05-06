use crate::EncodedLen;
use std::fmt::Debug;

/// Tests the `EncodedLen` trait.
pub fn test_encoded_len<T>(value: &T, expected: usize)
where
    T: EncodedLen + Debug,
{
    match value.encoded_len() {
        Ok(encoded_len) => {
            assert_eq!(encoded_len, expected, "value={:?}", value);
        }
        Err(error) => {
            assert!(false, "value={:?} error={:?}", value, error);
        }
    }
}
