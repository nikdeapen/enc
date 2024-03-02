use std::io;
use std::io::Write;

use crate::var_int::VarIntSize;
use crate::{EncodeToWrite, EncodedLen};

/// A value that can encode itself to a `Write` with a variable-length encoded length prefix.
pub trait EncodeToWriteLengthPrefixed: EncodedLen + EncodeToWrite {
    /// Encodes the value to the `Write` with a length prefix.
    fn encode_to_write_length_prefixed<W>(&self, w: &mut W) -> Result<usize, io::Error>
    where
        W: Write,
    {
        let encoded_len: usize = self.encoded_len();
        let prefix_len: usize = VarIntSize::from(encoded_len).encode_to_write(w)?;
        let also_encoded_len: usize = self.encode_to_write(w)?;
        debug_assert_eq!(encoded_len, also_encoded_len);
        Ok(prefix_len + encoded_len)
    }
}
