use crate::Error::{InsufficientTargetSpace, IntegerOverflow};
use crate::{data, Encoder, Error, StringEncoder};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// Responsible for encoding data in the hexadecimal format.
#[derive(Copy, Clone, Debug)]
pub struct HexEncoder {
    hex: &'static [u8; 16],
}

impl Ord for HexEncoder {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hex[10].cmp(&other.hex[10])
    }
}

impl PartialOrd for HexEncoder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for HexEncoder {}

impl PartialEq for HexEncoder {
    fn eq(&self, other: &Self) -> bool {
        self.hex[10].eq(&other.hex[10])
    }
}

impl Hash for HexEncoder {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hex[10].hash(state)
    }
}

impl HexEncoder {
    //! Special Encoders

    /// The lowercase hex encoder.
    pub const LOWER: Self = Self {
        hex: b"0123456789abcdef",
    };

    /// The uppercase hex encoder.
    pub const UPPER: Self = Self {
        hex: b"0123456789ABCDEF",
    };
}

impl HexEncoder {
    //! Encoding

    /// Encodes `b` as two hex bytes.
    #[inline(always)]
    pub const fn encode_bytes(&self, b: u8) -> (u8, u8) {
        (self.hex[(b as usize) >> 4], self.hex[(b as usize) & 0xF])
    }

    /// Encodes `b` as two hex chars.
    #[inline(always)]
    pub const fn encode_chars(&self, b: u8) -> (char, char) {
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

impl StringEncoder for HexEncoder {
    fn append_to_string(&self, data: &[u8], target: &mut String) -> Result<usize, Error> {
        unsafe { data::util::append_to_string_unchecked(self, data, target) }
    }
}

#[cfg(test)]
mod tests {
    use crate::hex::HexEncoder;
    use crate::{Error, StringEncoder};

    #[test]
    fn compare() {
        assert_eq!(HexEncoder::LOWER, HexEncoder::LOWER);
        assert_eq!(HexEncoder::UPPER, HexEncoder::UPPER);
        assert_ne!(HexEncoder::LOWER, HexEncoder::UPPER);
        assert!(HexEncoder::UPPER < HexEncoder::LOWER);
    }

    #[test]
    fn encode() -> Result<(), Error> {
        let test_cases: &[(&[u8], &str)] = &[
            (b"", ""),
            (b"\x01\x23\x45\x67\x89", "0123456789"),
            (b"\x10\x32\x54\x76\x98", "1032547698"),
            (b"\xAB\xCD\xEF", "abcdef"),
            (b"\xBA\xDC\xFE", "badcfe"),
        ];

        // todo -- encoder testing
        for (data, expected) in test_cases {
            let result: String = HexEncoder::LOWER.encode_as_string(data)?;
            assert_eq!(result, *expected);

            let result: String = HexEncoder::UPPER.encode_as_string(data)?;
            assert_eq!(result, expected.to_ascii_uppercase());
        }

        Ok(())
    }
}
