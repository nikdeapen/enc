use crate::{Encoder, Error};

/// Responsible for encoding binary data as text.
pub trait StringEncoder: Encoder {
    /// Appends the encoded data to the target string. Returns the length of the encoded data.
    fn encode_string(&self, data: &[u8], target: &mut String) -> Result<usize, Error>;

    /// Encodes the data as a string. Returns the string.
    fn encode_as_string(&self, data: &[u8]) -> Result<String, Error> {
        let mut string: String = String::default();
        self.encode_string(data, &mut string)?;
        Ok(string)
    }
}

/// Appends the encoded data to the target string. Returns the length of the encoded data. This
/// function does not ensure the encoded data is a valid UTF-8 byte sequence.
#[cfg(feature = "hex")]
pub(crate) unsafe fn encode_string_unchecked<E>(
    encoder: &E,
    data: &[u8],
    target: &mut String,
) -> Result<usize, Error>
    where
        E: Encoder,
{
    encoder.encode_vec(data, target.as_mut_vec())
}
