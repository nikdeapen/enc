use crate::{Encoder, Error};

/// Responsible for encoding binary data as UTF-8 encoded text.
pub trait StringEncoder: Encoder {
    /// Appends the encoded `data` to the `target` string.
    ///
    /// Returns the length of the encoded `data`.
    fn append_to_string(&self, data: &[u8], target: &mut String) -> Result<usize, Error>;

    /// Encodes the `data` as a string.
    ///
    /// Returns the string.
    fn encode_as_string(&self, data: &[u8]) -> Result<String, Error> {
        let mut string: String = String::default();
        self.append_to_string(data, &mut string)?;
        Ok(string)
    }
}

/// Appends the encoded `data` to the `target` string.
///
/// Returns the length of the encoded `data`.
///
/// # Safety
/// The encoded `data` must be a valid UTF-8 byte sequence.
#[allow(dead_code)]
pub(crate) unsafe fn append_to_string_unchecked<E>(
    encoder: &E,
    data: &[u8],
    target: &mut String,
) -> Result<usize, Error>
where
    E: Encoder,
{
    encoder.append_to_vec(data, unsafe { target.as_mut_vec() })
}
