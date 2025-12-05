use crate::hex::HexEncoder;
use crate::percent::SpecialSet;
use crate::Error::{InsufficientTargetSpace, IntegerOverflow};
use crate::{data, Encoder, Error, StringEncoder};

/// Responsible for encoding data in the URL percent encoded format.
#[derive(Copy, Clone, Debug)]
pub struct PercentEncoder {
    hex_encoder: HexEncoder,
    do_not_encode: SpecialSet,
}

impl PercentEncoder {
    //! Construction

    /// Creates a new percent encoder.
    pub const fn new(hex_encoder: HexEncoder, do_not_encode: SpecialSet) -> Self {
        Self {
            hex_encoder,
            do_not_encode,
        }
    }
}

impl<S: Into<SpecialSet>> From<S> for PercentEncoder {
    fn from(do_not_encode: S) -> Self {
        Self::new(HexEncoder::UPPER, do_not_encode.into())
    }
}

impl PercentEncoder {
    //! Encoding

    /// Checks if `c` needs encoding.
    #[inline(always)]
    pub fn needs_encoding(&self, c: u8) -> bool {
        !c.is_ascii_alphanumeric() && !self.do_not_encode.contains(c)
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

impl StringEncoder for PercentEncoder {
    fn append_to_string(&self, data: &[u8], target: &mut String) -> Result<usize, Error> {
        unsafe { data::util::append_to_string_unchecked(self, data, target) }
    }
}

#[cfg(test)]
mod tests {
    use crate::percent::percent_encoder::PercentEncoder;
    use crate::{Error, StringEncoder};

    #[test]
    fn encode() -> Result<(), Error> {
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

        // todo - encoder testing
        let encoder: PercentEncoder = "+-.".into();
        for (data, expected) in test_cases {
            let result: String = encoder.encode_as_string(data.as_bytes())?;
            assert_eq!(result, *expected);
        }

        Ok(())
    }
}
