use crate::Error::InsufficientTargetSpace;
use crate::hex::HexEncoder;
use crate::{Decoder, Error};

/// Responsible for decoding data in the hexadecimal format.
///
/// # Case
/// This decoder implementation is case-insensitive.
///
/// # Validation
/// This decoder implementation does nothing to validate the encoded data beyond requiring an even
/// number of encoded bytes. If invalid input data is given, the output bytes are undefined. The
/// decoded length calculation will still be accurate, and decoding data will not cause a panic.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
#[non_exhaustive]
pub struct HexDecoder {}

impl HexDecoder {
    //! Constants

    /// The case-insensitive decoding table. (indexed by the full byte range, so no masking is needed)
    const DECODING_TABLE: [u8; 256] = Self::create_decoding_table();

    /// Creates the case-insensitive decoding table. (the inverse of the hex encoding chars)
    const fn create_decoding_table() -> [u8; 256] {
        let mut t: [u8; 256] = [0x00; 256];

        let mut i: usize = 0;
        while i < 16 {
            t[HexEncoder::LOWER_CHARS[i] as usize] = i as u8;
            t[HexEncoder::UPPER_CHARS[i] as usize] = i as u8;
            i += 1;
        }

        t
    }
}

impl HexDecoder {
    //! Decoding

    /// Decodes the `high` and `low` hex bytes as a byte.
    ///
    /// If either input byte is invalid, the output byte is undefined.
    #[inline(always)]
    pub const fn decode_bytes(high: u8, low: u8) -> u8 {
        (Self::DECODING_TABLE[high as usize] << 4) | Self::DECODING_TABLE[low as usize]
    }

    /// Decodes the `high` and `low` hex chars as a byte.
    ///
    /// If either input char is invalid, the output byte is undefined.
    #[inline(always)]
    pub const fn decode_chars(high: char, low: char) -> u8 {
        Self::decode_bytes(((high as u32) & 0xFF) as u8, ((low as u32) & 0xFF) as u8)
    }
}

impl Decoder for HexDecoder {
    fn decoded_len(&self, data: &[u8]) -> Result<usize, Error> {
        if data.len().is_multiple_of(2) {
            Ok(data.len() / 2)
        } else {
            Err(Error::invalid_encoded_data_with(format!(
                "odd number of bytes: {}",
                data.len()
            )))
        }
    }

    fn decode_to_slice(&self, data: &[u8], target: &mut [u8]) -> Result<usize, Error> {
        let decoded_len: usize = self.decoded_len(data)?;
        if decoded_len > target.len() {
            Err(InsufficientTargetSpace)
        } else {
            let target: &mut [u8] = &mut target[..decoded_len];
            for (d, t) in data.as_chunks::<2>().0.iter().zip(target.iter_mut()) {
                *t = Self::decode_bytes(d[0], d[1])
            }
            Ok(decoded_len)
        }
    }
}

#[cfg(test)]
#[cfg(feature = "dev")]
mod tests {
    use crate::Decoder;
    use crate::hex::{HexDecoder, HexEncoder};
    use crate::test::{test_decoder, test_round_trip};

    /// Checks the generated decoding table against every hex char.
    #[test]
    fn decoding_table() {
        for b in 0u8..=255 {
            let expected: Option<u8> = match b {
                b'0'..=b'9' => Some(b - b'0'),
                b'a'..=b'f' => Some(b - b'a' + 10),
                b'A'..=b'F' => Some(b - b'A' + 10),
                _ => None,
            };
            if let Some(expected) = expected {
                assert_eq!(HexDecoder::decode_bytes(b'0', b), expected, "b={b:#04X}");
                assert_eq!(
                    HexDecoder::decode_bytes(b, b'0'),
                    expected << 4,
                    "b={b:#04X}"
                );
            }
        }
    }

    #[test]
    fn round_trip() {
        test_round_trip(&HexEncoder::LOWER, &HexDecoder::default(), 200);
        test_round_trip(&HexEncoder::UPPER, &HexDecoder::default(), 200);
    }

    #[test]
    fn decode_bytes_decode_chars() {
        let cases: &[(u8, u8, u8)] = &[
            (b'0', b'0', 0x00),
            (b'F', b'F', 0xFF),
            (b'a', b'b', 0xAB),
            (b'A', b'B', 0xAB),
            (b'a', b'B', 0xAB),
            (b'A', b'b', 0xAB),
            (b'0', b'9', 0x09),
            (b'f', b'f', 0xFF),
        ];
        for (high, low, expected) in cases {
            assert_eq!(HexDecoder::decode_bytes(*high, *low), *expected);
            assert_eq!(
                HexDecoder::decode_chars(*high as char, *low as char),
                *expected
            );
        }
    }

    #[test]
    fn decode_odd_length() {
        let decoder: HexDecoder = HexDecoder::default();
        let mut target: Vec<u8> = vec![0u8; 4];
        assert!(matches!(
            decoder.decode_to_slice(b"FFF", &mut target),
            Err(crate::Error::InvalidEncodedData { .. })
        ));
    }

    #[test]
    fn decode_insufficient_space() {
        let decoder: HexDecoder = HexDecoder::default();
        let mut target: Vec<u8> = vec![];
        assert!(matches!(
            decoder.decode_to_slice(b"FF", &mut target),
            Err(crate::Error::InsufficientTargetSpace)
        ));
    }

    #[test]
    fn decode() {
        let test_cases: &[(&str, &[u8])] = &[
            ("", b""),
            ("0123456789", b"\x01\x23\x45\x67\x89"),
            ("1032547698", b"\x10\x32\x54\x76\x98"),
            ("abcdef", b"\xAB\xCD\xEF"),
            ("badcfe", b"\xBA\xDC\xFE"),
            ("ABCDEF", b"\xAB\xCD\xEF"),
            ("BADCFE", b"\xBA\xDC\xFE"),
            ("aBcDeF", b"\xAB\xCD\xEF"),
            ("AbCdEf", b"\xAB\xCD\xEF"),
            ("BaDcFe", b"\xBA\xDC\xFE"),
            ("bAdCfE", b"\xBA\xDC\xFE"),
        ];
        test_decoder(&HexDecoder::default(), test_cases);
    }
}
