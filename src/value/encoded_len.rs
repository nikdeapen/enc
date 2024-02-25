/// A value with an encoded length.
pub trait EncodedLen {
    /// Gets the length of the encoded value.
    fn encoded_len(&self) -> usize;
}
