use crate::hex::{HexDecoder, HexValidator};
use crate::Error::InsufficientTargetSpace;
use crate::{Decoder, Error};

/// Responsible for decoding data in the URL percent encoded format.
///
/// # Validation
/// No validation is done on the decoded data. All properly encoded characters will be decoded and
/// improperly encoded characters will pass through unchanged.
#[derive(Copy, Clone, Default)]
pub struct PercentDecoder {
    hex_decoder: HexDecoder,
}

impl<H: Into<HexDecoder>> From<H> for PercentDecoder {
    fn from(hex_decoder: H) -> Self {
        Self {
            hex_decoder: hex_decoder.into(),
        }
    }
}

impl PercentDecoder {
    //! Decoding

    /// Checks if the prefix of `data` is a valid, encoded byte.
    #[inline(always)]
    fn prefix_is_encoded(data: &[u8]) -> bool {
        debug_assert!(!data.is_empty());
        debug_assert_eq!(data[0], b'%');

        data.len() >= 3
            && HexValidator::CASELESS.is_valid_byte(data[1])
            && HexValidator::CASELESS.is_valid_byte(data[2])
    }
}

impl Decoder for PercentDecoder {
    fn decoded_len(&self, data: &[u8]) -> Result<usize, Error> {
        let encoded: usize = data
            .iter()
            .enumerate()
            .filter(|(i, c)| **c == b'%' && Self::prefix_is_encoded(&data[*i..]))
            .count();
        Ok(data.len() - (2 * encoded))
    }

    fn decode_to_slice(&self, data: &[u8], target: &mut [u8]) -> Result<usize, Error> {
        let decoded_len: usize = self.decoded_len(data)?;
        if decoded_len > target.len() {
            Err(InsufficientTargetSpace)
        } else {
            let mut d: usize = 0;
            let mut t: usize = 0;
            while d < data.len() {
                let c: u8 = data[d];
                if c == b'%' && Self::prefix_is_encoded(&data[d..]) {
                    target[t] = self.hex_decoder.decode_bytes(data[d + 1], data[d + 2]);
                    t += 1;
                    d += 3;
                } else {
                    target[t] = c;
                    t += 1;
                    d += 1;
                }
            }
            debug_assert_eq!(t, decoded_len);
            Ok(t)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::percent::PercentDecoder;
    use crate::{Decoder, Error};

    #[test]
    fn decode() -> Result<(), Error> {
        let test_cases: &[(&str, &str)] = &[
            ("", ""),
            ("azAZ09+-. ", "azAZ09+-. "),
            ("你好", "你好"),
            ("%", "%"),
            ("%0", "%0"),
            ("%12", "\x12"),
            ("%0x", "%0x"),
            ("%x0", "%x0"),
            ("%%00", "%\x00"),
        ];
        let decoder: PercentDecoder = PercentDecoder::default();
        for (data, decoded) in test_cases {
            let result: Vec<u8> = decoder.decode_as_vec(data.as_bytes())?;
            let result: String = String::from_utf8(result).unwrap();
            assert_eq!(result, *decoded);
        }
        Ok(())
    }
}
