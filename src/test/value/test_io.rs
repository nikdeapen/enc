use crate::test::{test_decode, test_encode};
use crate::{DecodeFromRead, DecodeFromReadPrefix, EncodeToSlice, EncodeToWrite, EncodedLen};
use std::fmt::Debug;

/// Tests the encoding & decoding traits.
pub fn test_io<T>(value: &T, encoded: &[u8], add_len_prefix: bool)
where
    T: EncodedLen
        + EncodeToSlice
        + EncodeToWrite
        + DecodeFromRead
        + DecodeFromReadPrefix
        + PartialEq
        + Debug,
{
    test_encode(value, encoded);
    test_decode(encoded, value, add_len_prefix);
}
