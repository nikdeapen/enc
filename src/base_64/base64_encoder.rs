use crate::data::append_to_string_unchecked;
use crate::Error::{InsufficientTargetSpace, IntegerOverflow};
use crate::{Encoder, Error, TextEncoder};

/// Responsible for encoding data in the hexadecimal format.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Base64Encoder {
    v63: u8,
    v64: u8,
    padding: Option<u8>,
}

impl Base64Encoder {
    //! Construction

    /// Creates a new base-64 encoder.
    pub const fn new(v63: u8, v64: u8, padding: Option<u8>) -> Option<Self> {
        if Self::is_valid_config(v63, v64, padding) {
            Some(Self { v63, v64, padding })
        } else {
            None
        }
    }
}

impl Default for Base64Encoder {
    fn default() -> Self {
        Self {
            v63: b'+',
            v64: b'/',
            padding: Some(b'='),
        }
    }
}

impl Base64Encoder {
    //! Validation

    /// Checks if the encoding config is valid.
    pub const fn is_valid_config(v63: u8, v64: u8, padding: Option<u8>) -> bool {
        v63.is_ascii_punctuation()
            && v64.is_ascii_punctuation()
            && v63 != v64
            && (if let Some(padding) = padding {
                padding.is_ascii_punctuation() && padding != v63 && padding != v64
            } else {
                true
            })
    }
}

impl Base64Encoder {
    //! Constants

    /// The encoding table.
    const ENCODING_TABLE: &'static [u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
}

impl Base64Encoder {
    //! Block Encoding

    /// Encodes a single full block of data. Will encode 3 data bytes into 4 target bytes.
    #[inline(always)]
    fn encode_block(data: &[u8], target: &mut [u8]) {
        debug_assert!(data.len() >= 3);
        debug_assert!(target.len() >= 4);

        let a: u32 = data[0] as u32;
        let b: u32 = data[1] as u32;
        let c: u32 = data[2] as u32;
        let bits: u32 = (a << 16) | (b << 8) | c;

        let ai: usize = (bits >> 18) as usize;
        let bi: usize = ((bits >> 12) & 0x3F) as usize;
        let ci: usize = ((bits >> 6) & 0x3F) as usize;
        let di: usize = (bits & 0x3F) as usize;

        target[0] = Self::ENCODING_TABLE[ai];
        target[1] = Self::ENCODING_TABLE[bi];
        target[2] = Self::ENCODING_TABLE[ci];
        target[3] = Self::ENCODING_TABLE[di];
    }

    /// Encodes the last block with one byte of data.
    /// Returns the number of encoded bytes: (2 or 4).
    #[inline(always)]
    fn encode_last_1(&self, data: &[u8], target: &mut [u8]) -> usize {
        debug_assert_eq!(data.len(), 1);
        debug_assert!(target.len() >= 2);

        let bits: u32 = data[0] as u32;

        let ai: usize = (bits >> 2) as usize;
        let bi: usize = ((bits << 4) & 0x3F) as usize;

        target[0] = Self::ENCODING_TABLE[ai];
        target[1] = Self::ENCODING_TABLE[bi];

        if let Some(padding) = self.padding {
            debug_assert_eq!(target.len(), 4);
            target[2] = padding;
            target[3] = padding;
            4
        } else {
            debug_assert_eq!(target.len(), 2);
            2
        }
    }

    /// Encodes the last block with two bytes of data.
    /// Returns the number of encoded bytes: (3 or 4).
    #[inline(always)]
    fn encode_last_2(&self, data: &[u8], target: &mut [u8]) -> usize {
        debug_assert_eq!(data.len(), 2);
        debug_assert!(target.len() >= 3);

        let a: u32 = data[0] as u32;
        let b: u32 = data[1] as u32;
        let bits: u32 = (a << 8) | b;

        let ai: usize = (bits >> 10) as usize;
        let bi: usize = ((bits >> 4) & 0x3F) as usize;
        let ci: usize = ((bits << 2) & 0x3F) as usize;

        target[0] = Self::ENCODING_TABLE[ai];
        target[1] = Self::ENCODING_TABLE[bi];
        target[2] = Self::ENCODING_TABLE[ci];

        if let Some(padding) = self.padding {
            debug_assert_eq!(target.len(), 4);
            target[3] = padding;
            4
        } else {
            debug_assert_eq!(target.len(), 3);
            3
        }
    }
}

impl Base64Encoder {
    //! Correction

    /// Corrects the last two encoding bytes. (makes non-default configs slower but ¯\_(ツ)_/¯)
    #[inline(always)]
    fn correct_v63_and_v64(&self, target: &mut [u8]) {
        if self.v63 != b'+' || self.v64 != b'/' {
            target.iter_mut().for_each(|t| {
                if *t == b'+' {
                    *t = self.v63
                } else if *t == b'/' {
                    *t = self.v64
                }
            });
        }
    }
}

impl Encoder for Base64Encoder {
    fn encoded_len(&self, data: &[u8]) -> Result<usize, Error> {
        let div: usize = data.len() / 3;
        let rem: usize = data.len() % 3;
        let extra: usize = match rem {
            0 => 0,
            1 => {
                if self.padding.is_some() {
                    4
                } else {
                    2
                }
            }
            2 => {
                if self.padding.is_some() {
                    4
                } else {
                    3
                }
            }
            _ => unreachable!(),
        };
        div.checked_mul(4)
            .ok_or(IntegerOverflow)?
            .checked_add(extra)
            .ok_or(IntegerOverflow)
    }

    fn encode_to_slice(&self, data: &[u8], target: &mut [u8]) -> Result<usize, Error> {
        let encoded_len: usize = self.encoded_len(data)?;
        if encoded_len > target.len() {
            Err(InsufficientTargetSpace)
        } else {
            let target: &mut [u8] = &mut target[..encoded_len];
            let div: usize = data.len() / 3;
            let rem: usize = data.len() % 3;
            let mut d: usize = 0;
            let mut t: usize = 0;
            for _ in 0..div {
                Self::encode_block(&data[d..], &mut target[t..]);
                d += 3;
                t += 4;
            }
            match rem {
                0 => {
                    self.correct_v63_and_v64(target);
                }
                1 => {
                    let extra: usize = self.encode_last_1(&data[d..], &mut target[t..]);
                    self.correct_v63_and_v64(&mut target[t + 2..]);
                    t += extra;
                }
                2 => {
                    let extra: usize = self.encode_last_2(&data[d..], &mut target[t..]);
                    self.correct_v63_and_v64(&mut target[t + 3..]);
                    t += extra;
                }
                _ => unreachable!(),
            }
            debug_assert_eq!(encoded_len, t);
            Ok(encoded_len)
        }
    }
}

impl TextEncoder for Base64Encoder {
    fn append_to_string(&self, data: &[u8], target: &mut String) -> Result<usize, Error> {
        unsafe { append_to_string_unchecked(self, data, target) }
    }
}

#[cfg(test)]
mod tests {
    use crate::base_64::Base64Encoder;
    use crate::TextEncoder;

    #[test]
    fn encode() -> Result<(), crate::Error> {
        let test_cases: &[(&[u8], &str)] = &[
            (b"\x00\x10\x83", "ABCD"),
            (b"\x10\x51\x87", "EFGH"),
            (b"\x20\x92\x8B", "IJKL"),
            (b"\x30\xD3\x8F", "MNOP"),
            (b"\x41\x14\x93", "QRST"),
            (b"\x51\x55\x97", "UVWX"),
            (b"\x61\x96\x9B", "YZab"),
            (b"\x71\xD7\x9F", "cdef"),
            (b"\x82\x18\xA3", "ghij"),
            (b"\x92\x59\xA7", "klmn"),
            (b"\xA2\x9A\xAB", "opqr"),
            (b"\xB2\xDB\xAF", "stuv"),
            (b"\xC3\x1C\xB3", "wxyz"),
            (b"\xD3\x5D\xB7", "0123"),
            (b"\xE3\x9E\xBB", "4567"),
            (b"\xF3\xDF\xBF", "89+/"),
            (b"", ""),
            (b"\x00", "AA=="),
            (b"\xFF", "/w=="),
            (b"\x00\x00", "AAA="),
            (b"\xFF\xFF", "//8="),
            (b"\x00\x00\x00", "AAAA"),
            (b"\xFF\xFF\xFF", "////"),
            (b"\x00\x00\x00\x00", "AAAAAA=="),
            (b"\xFF\xFF\xFF\xFF", "/////w=="),
            (b"\x00\x00\x00\x00\x00", "AAAAAAA="),
            (b"\xFF\xFF\xFF\xFF\xFF", "//////8="),
            (b"\x00\x00\x00\x00\x00\x00", "AAAAAAAA"),
            (b"\xFF\xFF\xFF\xFF\xFF\xFF", "////////"),
        ];
        let encoder: Base64Encoder = Base64Encoder::default();
        for (data, expected) in test_cases {
            let result: String = encoder.encode_as_string(*data)?;
            assert_eq!(result, *expected, "data={:?}", *data);
        }
        Ok(())
    }
}
