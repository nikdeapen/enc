use std::io;

/// A value that can encode itself to a `Write`.
pub trait EncodeToWrite {
    /// Encodes the value to the `Write`.
    ///
    /// Returns the length of the encoded value.
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, io::Error>
    where
        W: io::Write;
}
