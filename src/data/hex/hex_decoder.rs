use crate::Error::{InsufficientTargetSpace, InvalidEncodedData};
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
pub struct HexDecoder {
    _nothing: (),
}

impl HexDecoder {
    //! Constants

    /// The case-insensitive decoding table.
    const DECODING_TABLE: [u8; 128] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
}

impl HexDecoder {
    //! Decoding

    /// Decodes the `high` and `low` hex bytes as a byte.
    ///
    /// If either input byte is invalid, the output byte is undefined.
    #[inline(always)]
    pub const fn decode_bytes(high: u8, low: u8) -> u8 {
        (Self::DECODING_TABLE[(high & 0x7F) as usize] << 4)
            | Self::DECODING_TABLE[(low & 0x7F) as usize]
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
        let div: usize = data.len() / 2;
        let rem: usize = data.len() % 2;
        if rem == 1 {
            Err(InvalidEncodedData {
                reason: Some(format!("odd number of bytes: {}", data.len()).into()),
            })
        } else {
            Ok(div)
        }
    }

    fn decode_to_slice(&self, data: &[u8], target: &mut [u8]) -> Result<usize, Error> {
        let decoded_len: usize = self.decoded_len(data)?;
        if decoded_len > target.len() {
            Err(InsufficientTargetSpace)
        } else {
            let target: &mut [u8] = &mut target[..decoded_len];
            for (d, t) in data.chunks_exact(2).zip(target.iter_mut()) {
                *t = Self::decode_bytes(d[0], d[1])
            }
            Ok(decoded_len)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hex::HexDecoder;
    use crate::{Decoder, Error};

    #[test]
    fn decode() -> Result<(), Error> {
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

        // todo -- decoder testing
        for (input, expected) in test_cases {
            let result: Vec<u8> = HexDecoder::default().decode_as_vec(input.as_bytes())?;
            assert_eq!(result, *expected, "input={}", *input);
        }

        Ok(())
    }
}
