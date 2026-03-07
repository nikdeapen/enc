use crate::hex::{HexDecoder, HexValidator};
use crate::Error::InsufficientTargetSpace;
use crate::{Decoder, Error};

/// Responsible for decoding data in the URL percent encoded format.
///
/// # Case
/// This decoder is case-insensitive. All encoded `%XX %xx %xX %Xx` sequences will be decoded.
///
/// # Validation
/// No validation is done on the encoded data. All properly encoded chars will be decoded, and
/// improperly encoded chars will pass through the decoder unchanged.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct PercentDecoder {
    _nothing: (),
}

impl PercentDecoder {
    //! Decoding

    /// Checks if the prefix of `data` is a valid, encoded byte.
    ///
    /// # Note
    /// The `data` must not be empty and the first byte must be the `%` symbol.
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
            .filter(|(_, c)| **c == b'%')
            .filter(|(i, _)| Self::prefix_is_encoded(&data[*i..]))
            .count();
        Ok(data.len() - (encoded * 2))
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
                    target[t] = HexDecoder::decode_bytes(data[d + 1], data[d + 2]);
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
#[cfg(feature = "dev")]
mod tests {
    use crate::percent::PercentDecoder;
    use crate::test::test_decoder;
    use crate::Decoder;

    #[test]
    fn decode() {
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
            ("%GG", "%GG"),
            ("a%GG", "a%GG"),
        ];
        let decoder: PercentDecoder = PercentDecoder::default();
        test_decoder(&decoder, test_cases);
    }

    #[test]
    fn decode_insufficient_space() {
        let decoder: PercentDecoder = PercentDecoder::default();
        let mut target: Vec<u8> = vec![];
        assert!(matches!(
            decoder.decode_to_slice(b"%FF", &mut target),
            Err(crate::Error::InsufficientTargetSpace)
        ));
    }
}
