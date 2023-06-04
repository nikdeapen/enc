use crate::hex::HexEncoder;
use crate::percent::SpecialSet;
use crate::Error::{InsufficientTargetSpace, IntegerOverflow};
use crate::{encode_string_unchecked, Encoder, Error, StringEncoder};

/// Responsible for percent encoding data.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct PercentEncoder {
    do_not_encode: SpecialSet,
    hex_encoder: HexEncoder,
}

impl<S: Into<SpecialSet>> From<S> for PercentEncoder {
    fn from(do_not_encode: S) -> Self {
        Self {
            do_not_encode: do_not_encode.into(),
            hex_encoder: HexEncoder::UPPER,
        }
    }
}

impl PercentEncoder {
    //! Encoding

    /// Checks if the byte needs encoding.
    #[inline(always)]
    pub fn needs_encoding(&self, b: u8) -> bool {
        !(b.is_ascii_alphanumeric() || self.do_not_encode.contains(b))
    }
}

impl Encoder for PercentEncoder {
    fn encoded_len(&self, data: &[u8]) -> Result<usize, Error> {
        let need_encoding: usize = data.iter().filter(|c| self.needs_encoding(**c)).count();
        let extra_bytes: usize = need_encoding.checked_mul(2).ok_or(IntegerOverflow)?;
        data.len().checked_add(extra_bytes).ok_or(IntegerOverflow)
    }

    fn encode_slice(&self, data: &[u8], target: &mut [u8]) -> Result<usize, Error> {
        let encoded_len: usize = self.encoded_len(data)?;
        if target.len() < encoded_len {
            Err(InsufficientTargetSpace)
        } else {
            let mut t: usize = 0;
            for d in data {
                if self.needs_encoding(*d) {
                    target[t] = b'%';
                    t += 1;
                    let (high, low) = self.hex_encoder.encode_as_bytes(*d);
                    target[t] = high;
                    t += 1;
                    target[t] = low;
                    t += 1;
                } else {
                    target[t] = *d;
                    t += 1;
                }
            }
            debug_assert_eq!(t, encoded_len);
            Ok(encoded_len)
        }
    }
}

impl StringEncoder for PercentEncoder {
    fn encode_string(&self, data: &[u8], target: &mut String) -> Result<usize, Error> {
        unsafe { encode_string_unchecked(self, data, target) }
    }
}
