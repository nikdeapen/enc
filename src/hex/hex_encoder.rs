use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

use crate::Error::{InsufficientTargetSpace, IntegerOverflow};
use crate::{encode_string_unchecked, Encoder, Error, StringEncoder};

/// Responsible for hex encoding data.
#[derive(Copy, Clone, Debug)]
pub struct HexEncoder {
    encoding: &'static [u8; 16],
}

impl HexEncoder {
    //! Special Encoders

    /// The lowercase hex encoder.
    pub const LOWER: Self = Self {
        encoding: b"0123456789abcdef",
    };

    /// The uppercase hex encoder.
    pub const UPPER: Self = Self {
        encoding: b"0123456789ABCDEF",
    };
}

impl Ord for HexEncoder {
    fn cmp(&self, other: &Self) -> Ordering {
        self.encoding[10].cmp(&other.encoding[10])
    }
}

impl PartialOrd for HexEncoder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.encoding[10].partial_cmp(&other.encoding[10])
    }
}

impl Eq for HexEncoder {}

impl PartialEq for HexEncoder {
    fn eq(&self, other: &Self) -> bool {
        self.encoding[10].eq(&other.encoding[10])
    }
}

impl Hash for HexEncoder {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u8(self.encoding[10]);
    }
}

impl HexEncoder {
    //! Encoding

    /// Encodes the byte as two hex bytes.
    #[inline(always)]
    pub const fn encode_as_bytes(&self, b: u8) -> (u8, u8) {
        (
            self.encoding[b as usize >> 4],
            self.encoding[b as usize & 0xF],
        )
    }

    /// Encodes the byte as two hex chars.
    #[inline(always)]
    pub const fn encode_as_chars(&self, b: u8) -> (char, char) {
        let (a, b) = self.encode_as_bytes(b);
        (a as char, b as char)
    }
}

impl Encoder for HexEncoder {
    fn encoded_len(&self, data: &[u8]) -> Result<usize, Error> {
        data.len().checked_mul(2).ok_or(IntegerOverflow)
    }

    fn encode_slice(&self, data: &[u8], target: &mut [u8]) -> Result<usize, Error> {
        let encoded_len: usize = self.encoded_len(data)?;
        if target.len() < encoded_len {
            Err(InsufficientTargetSpace)
        } else {
            let target: &mut [u8] = &mut target[..encoded_len];
            for (d, t) in data.iter().zip(target.chunks_exact_mut(2)) {
                let (high, low) = self.encode_as_bytes(*d);
                t[0] = high;
                t[1] = low;
            }
            Ok(encoded_len)
        }
    }
}

impl StringEncoder for HexEncoder {
    fn encode_string(&self, data: &[u8], target: &mut String) -> Result<usize, Error> {
        unsafe { encode_string_unchecked(self, data, target) }
    }
}
