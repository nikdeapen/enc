use crate::Error::{InsufficientTargetSpace, IntegerOverflow};
use crate::data::util::append_to_string_unchecked;
use crate::hex::HexEncoder;
use crate::percent::SpecialSet;
use crate::{Encoder, Error, StringEncoder, append_to_vec_with};

/// Responsible for encoding data in the URL percent encoded format.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct PercentEncoder {
    hex_encoder: HexEncoder,
    encoding_not_needed: SpecialSet,
}

impl PercentEncoder {
    //! Construction

    /// Creates a new percent encoder.
    pub const fn new(hex_encoder: HexEncoder, encoding_not_needed: SpecialSet) -> Self {
        Self {
            hex_encoder,
            encoding_not_needed,
        }
    }
}

/// Creates a percent encoder with the uppercase hex encoder.
impl<S: Into<SpecialSet>> From<S> for PercentEncoder {
    fn from(encoding_not_needed: S) -> Self {
        Self::new(HexEncoder::UPPER, encoding_not_needed.into())
    }
}

impl PercentEncoder {
    //! Encoding

    /// Checks if `c` needs encoding.
    #[inline(always)]
    pub fn needs_encoding(self, c: u8) -> bool {
        !c.is_ascii_alphanumeric() && !self.encoding_not_needed.contains(c)
    }

    /// Encodes the `data` into the `target` slice with the pre-computed `encoded_len`.
    ///
    /// Returns the length of the encoded `data`.
    ///
    /// The `encoded_len` must be the result of `encoded_len(data)`. Computing it is a full scan of the `data`, so
    /// callers that already have it pass it in rather than paying for a second scan.
    fn encode_to_slice_with_len(
        self,
        data: &[u8],
        encoded_len: usize,
        target: &mut [u8],
    ) -> Result<usize, Error> {
        if encoded_len > target.len() {
            Err(InsufficientTargetSpace)
        } else {
            let mut t: usize = 0;
            for c in data {
                if self.needs_encoding(*c) {
                    target[t] = b'%';
                    t += 1;
                    let (a, b) = self.hex_encoder.encode_bytes(*c);
                    target[t] = a;
                    t += 1;
                    target[t] = b;
                    t += 1;
                } else {
                    target[t] = *c;
                    t += 1;
                }
            }
            debug_assert_eq!(encoded_len, t);
            Ok(t)
        }
    }
}

impl Encoder for PercentEncoder {
    fn encoded_len(&self, data: &[u8]) -> Result<usize, Error> {
        let need_encoding: usize = data.iter().filter(|c| self.needs_encoding(**c)).count();
        data.len()
            .checked_add(need_encoding.checked_mul(2).ok_or(IntegerOverflow)?)
            .ok_or(IntegerOverflow)
    }

    fn encode_to_slice(&self, data: &[u8], target: &mut [u8]) -> Result<usize, Error> {
        let encoded_len: usize = self.encoded_len(data)?;
        self.encode_to_slice_with_len(data, encoded_len, target)
    }

    fn append_to_vec(&self, data: &[u8], target: &mut Vec<u8>) -> Result<usize, Error> {
        append_to_vec_with(
            target,
            || self.encoded_len(data),
            |encoded_len, t| self.encode_to_slice_with_len(data, encoded_len, t),
        )
    }
}

impl StringEncoder for PercentEncoder {
    fn append_to_string(&self, data: &[u8], target: &mut String) -> Result<usize, Error> {
        unsafe { append_to_string_unchecked(self, data, target) }
    }
}

#[cfg(test)]
#[cfg(feature = "dev")]
mod tests {
    use crate::Encoder;
    use crate::percent::percent_encoder::PercentEncoder;
    use crate::test::test_string_encoder;

    #[test]
    fn encode_insufficient_space() {
        let encoder: PercentEncoder = "+-.".into();
        let mut target: Vec<u8> = vec![0u8; 2];
        // b"\xFF" needs "%FF" = 3 bytes
        assert!(matches!(
            encoder.encode_to_slice(b"\xFF", &mut target),
            Err(crate::Error::InsufficientTargetSpace)
        ));
    }

    #[test]
    fn encode() {
        let test_cases: &[(&str, &str)] = &[
            ("", ""),
            ("az", "az"),
            ("AZ", "AZ"),
            ("09", "09"),
            ("\x00\x1F", "%00%1F"),
            (" ", "%20"),
            ("+-.", "+-."),
            ("!@`~", "%21%40%60%7E"),
            ("你好", "%E4%BD%A0%E5%A5%BD"),
        ];
        let encoder: PercentEncoder = "+-.".into();
        test_string_encoder(&encoder, test_cases);
    }
}
