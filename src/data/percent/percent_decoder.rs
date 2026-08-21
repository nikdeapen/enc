use crate::Error::InsufficientTargetSpace;
use crate::hex::{HexDecoder, HexValidator};
use crate::percent::prefix_is_encoded::prefix_is_encoded;
use crate::{Decoder, Error, append_to_vec_with};

/// Responsible for decoding data in the URL percent encoded format.
///
/// # Case
/// This decoder is case-insensitive. All encoded `%XX %xx %xX %Xx` sequences will be decoded.
///
/// # Validation
/// No validation is done on the encoded data. All properly encoded chars will be decoded, and
/// improperly encoded chars will pass through the decoder unchanged.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
#[non_exhaustive]
pub struct PercentDecoder {}

impl PercentDecoder {
    //! Decoding

    /// Decodes the `data` into the `target` slice with the pre-computed `decoded_len`.
    ///
    /// Returns the length of the decoded `data`.
    ///
    /// The `decoded_len` must be the result of `decoded_len(data)`. Computing it is a full scan of the `data`, so
    /// callers that already have it pass it in rather than paying for a second scan.
    fn decode_to_slice_with_len(
        data: &[u8],
        decoded_len: usize,
        target: &mut [u8],
    ) -> Result<usize, Error> {
        if decoded_len > target.len() {
            Err(InsufficientTargetSpace)
        } else {
            let mut d: usize = 0;
            let mut t: usize = 0;
            while d < data.len() {
                let c: u8 = data[d];
                if c == b'%' && prefix_is_encoded(HexValidator::CASELESS, &data[d..]) {
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

impl Decoder for PercentDecoder {
    fn decoded_len(&self, data: &[u8]) -> Result<usize, Error> {
        let encoded: usize = data
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == b'%')
            .filter(|(i, _)| prefix_is_encoded(HexValidator::CASELESS, &data[*i..]))
            .count();
        Ok(data.len() - (encoded * 2))
    }

    fn decode_to_slice(&self, data: &[u8], target: &mut [u8]) -> Result<usize, Error> {
        let decoded_len: usize = self.decoded_len(data)?;
        Self::decode_to_slice_with_len(data, decoded_len, target)
    }

    fn append_to_vec(&self, data: &[u8], target: &mut Vec<u8>) -> Result<usize, Error> {
        append_to_vec_with(
            target,
            || self.decoded_len(data),
            |decoded_len, t| Self::decode_to_slice_with_len(data, decoded_len, t),
        )
    }
}

#[cfg(test)]
#[cfg(feature = "dev")]
mod tests {
    use crate::Decoder;
    use crate::percent::{PercentDecoder, PercentEncoder};
    use crate::test::{test_decoder, test_round_trip};

    #[test]
    fn round_trip() {
        let encoder: PercentEncoder = "+-.".into();
        test_round_trip(&encoder, &PercentDecoder::default(), 200);
    }

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
