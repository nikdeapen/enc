use crate::test::{
    test_decode_from_read, test_decode_from_read_prefix, test_encode_to_slice,
    test_encode_to_write, test_encoded_len,
};
use crate::{DecodeFromRead, DecodeFromReadPrefix, EncodeToSlice, EncodeToWrite};
use std::fmt::Debug;

/// Tests encoding and decoding the `value`.
pub fn test_io<T>(value: &T, encoded: &[u8], add_len_prefix: bool)
where
    T: EncodeToSlice + EncodeToWrite + DecodeFromRead + DecodeFromReadPrefix + PartialEq + Debug,
{
    test_encoded_len(value, encoded.len());
    test_encode_to_slice(value, encoded);
    test_encode_to_write(value, encoded);
    test_decode_from_read(encoded, value);
    test_decode_from_read_prefix(encoded, value, add_len_prefix);
}
