use crate::data::append_to_string_unchecked;
use crate::Error::{InsufficientTargetSpace, IntegerOverflow};
use crate::{Encoder, Error, TextEncoder};

/// Responsible for encoding data in the hexadecimal format.
#[derive(Copy, Clone, Debug)]
pub struct HexEncoder {
    uppercase: bool,
}

impl HexEncoder {
    //! Special Encoders

    /// The lowercase hex encoder.
    pub const LOWER: Self = Self { uppercase: false };

    /// The uppercase hex encoder.
    pub const UPPER: Self = Self { uppercase: true };
}

impl HexEncoder {
    //! Constants

    /// The lowercase hex chars.
    const CHARS_LOWER: [u8; 16] = *b"0123456789abcdef";

    /// The uppercase hex chars.
    const CHARS_UPPER: [u8; 16] = *b"0123456789ABCDEF";
}

impl HexEncoder {
    //! Encoding

    /// Gets the 16 hexadecimal chars of the proper case.
    #[inline(always)]
    const fn hex(&self) -> &[u8; 16] {
        if self.uppercase {
            &Self::CHARS_UPPER
        } else {
            &Self::CHARS_LOWER
        }
    }

    /// Encodes the byte as two hex chars.
    #[inline(always)]
    pub fn encode_bytes(&self, b: u8) -> (u8, u8) {
        let hex: &[u8; 16] = self.hex();
        (hex[(b as usize) >> 4], hex[(b as usize) & 0xF])
    }

    /// Encodes the byte as two hex chars.
    #[inline(always)]
    pub fn encode_chars(&self, b: u8) -> (char, char) {
        let (a, b) = self.encode_bytes(b);
        (a as char, b as char)
    }
}

impl Encoder for HexEncoder {
    fn encoded_len(&self, data: &[u8]) -> Result<usize, Error> {
        data.len().checked_mul(2).ok_or(IntegerOverflow)
    }

    fn encode_to_slice(&self, data: &[u8], target: &mut [u8]) -> Result<usize, Error> {
        let encoded_len: usize = self.encoded_len(data)?;
        if encoded_len > target.len() {
            Err(InsufficientTargetSpace)
        } else {
            let target: &mut [u8] = &mut target[..encoded_len];
            for (d, t) in data.iter().zip(target.chunks_exact_mut(2)) {
                let (a, b) = self.encode_bytes(*d);
                t[0] = a;
                t[1] = b;
            }
            Ok(encoded_len)
        }
    }
}

impl TextEncoder for HexEncoder {
    fn append_to_string(&self, data: &[u8], target: &mut String) -> Result<usize, Error> {
        unsafe { append_to_string_unchecked(self, data, target) }
    }
}

#[cfg(test)]
mod tests {
    use crate::hex::HexEncoder;
    use crate::{Error, TextEncoder};

    #[test]
    fn encode() -> Result<(), Error> {
        let test_cases: &[(&[u8], &str)] = &[
            (b"", ""),
            (b"\x01\x23\x45\x67\x89", "0123456789"),
            (b"\x10\x32\x54\x76\x98", "1032547698"),
            (b"\xAB\xCD\xEF", "abcdef"),
            (b"\xBA\xDC\xFE", "badcfe"),
        ];
        for (data, expected) in test_cases {
            let result: String = HexEncoder::LOWER.encode_as_string(*data)?;
            assert_eq!(result, *expected);

            let result: String = HexEncoder::UPPER.encode_as_string(*data)?;
            assert_eq!(result, expected.to_ascii_uppercase());
        }
        Ok(())
    }
}
