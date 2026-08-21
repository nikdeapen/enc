use crate::Error::InsufficientTargetSpace;
use crate::base_64::as_array::as_array;
use crate::base_64::constants::{DECODED_BLOCK_SIZE, ENCODED_BLOCK_SIZE};
use crate::base_64::encode;
use crate::base_64::encode::EncodingTable;
use crate::data::util::append_to_string_unchecked;
use crate::{Encoder, Error, StringEncoder};
use std::fmt::{Debug, Formatter};

/// Responsible for encoding data in the base-64 format.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Base64Encoder {
    table: EncodingTable,
    padding: Option<u8>,
}

impl Base64Encoder {
    //! Validation

    /// Checks if the encoding config is valid.
    pub const fn is_valid_config(v63: u8, v64: u8, padding: Option<u8>) -> bool {
        (v63.is_ascii_punctuation() && v64.is_ascii_punctuation())
            && v63 != v64
            && (if let Some(padding) = padding {
                padding.is_ascii_punctuation() && padding != v63 && padding != v64
            } else {
                true
            })
    }
}

impl Base64Encoder {
    //! Construction

    /// Creates a new base-64 encoder.
    ///
    /// Returns `None` if the encoding config is invalid.
    pub fn new(v63: u8, v64: u8, padding: Option<u8>) -> Option<Self> {
        if Self::is_valid_config(v63, v64, padding) {
            Some(Self {
                table: EncodingTable::get_encoding_table(v63, v64),
                padding,
            })
        } else {
            None
        }
    }
}

impl Default for Base64Encoder {
    fn default() -> Self {
        Self {
            table: EncodingTable::default(),
            padding: Self::DEFAULT_PADDING,
        }
    }
}

impl Base64Encoder {
    //! Special Encoders

    /// Gets the URL-safe encoder.
    pub fn url_safe_encoder() -> Self {
        Self::new(
            Self::URL_SAFE_V63,
            Self::URL_SAFE_V64,
            Self::URL_SAFE_PADDING,
        )
        .unwrap()
    }
}

impl Encoder for Base64Encoder {
    fn encoded_len(&self, data: &[u8]) -> Result<usize, Error> {
        encode::encoded_len(data.len(), self.padding.is_some())
    }

    fn encode_to_slice(&self, data: &[u8], target: &mut [u8]) -> Result<usize, Error> {
        let encoded_len: usize = self.encoded_len(data)?;
        if encoded_len > target.len() {
            Err(InsufficientTargetSpace)
        } else {
            let target: &mut [u8] = &mut target[..encoded_len];
            let table: &[u8; 64] = self.table.table();

            let (blocks, last_block) = data.as_chunks::<DECODED_BLOCK_SIZE>();
            let (block_targets, last_target) =
                target.split_at_mut(blocks.len() * ENCODED_BLOCK_SIZE);
            for (block, block_target) in blocks.iter().zip(
                block_targets
                    .as_chunks_mut::<ENCODED_BLOCK_SIZE>()
                    .0
                    .iter_mut(),
            ) {
                encode::encode_block(table, block, block_target);
            }

            let mut t: usize = blocks.len() * ENCODED_BLOCK_SIZE;
            match last_block.len() {
                0 => {}
                1 => {
                    t += encode::encode_last_block_1(
                        table,
                        self.padding,
                        as_array(last_block),
                        last_target,
                    );
                }
                2 => {
                    t += encode::encode_last_block_2(
                        table,
                        self.padding,
                        as_array(last_block),
                        last_target,
                    );
                }
                _ => unreachable!(),
            }
            debug_assert_eq!(encoded_len, t);
            Ok(encoded_len)
        }
    }
}

impl StringEncoder for Base64Encoder {
    fn append_to_string(&self, data: &[u8], target: &mut String) -> Result<usize, Error> {
        unsafe { append_to_string_unchecked(self, data, target) }
    }
}

impl Debug for Base64Encoder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (v63, v64) = self.table.values();
        f.debug_struct("Base64Encoder")
            .field("v63", &(v63 as char))
            .field("v64", &(v64 as char))
            .field("padding", &self.padding.map(|p| p as char))
            .finish()
    }
}

#[cfg(test)]
#[cfg(feature = "dev")]
mod tests {
    use crate::StringEncoder;
    use crate::base_64::Base64Encoder;
    use crate::test::test_string_encoder;

    #[test]
    fn new() {
        // valid configs
        assert!(Base64Encoder::new(b'+', b'/', Some(b'=')).is_some());
        assert!(Base64Encoder::new(b'-', b'_', None).is_some());
        assert!(Base64Encoder::new(b'!', b'@', Some(b'#')).is_some());

        // the 63rd & 64th values must be distinct ASCII punctuation
        assert!(Base64Encoder::new(b'+', b'+', None).is_none());
        assert!(Base64Encoder::new(b'a', b'/', None).is_none());
        assert!(Base64Encoder::new(b'+', b'9', None).is_none());
        assert!(Base64Encoder::new(0x80, b'/', None).is_none());

        // the padding must differ from both
        assert!(Base64Encoder::new(b'+', b'/', Some(b'+')).is_none());
        assert!(Base64Encoder::new(b'+', b'/', Some(b'/')).is_none());
        assert!(Base64Encoder::new(b'+', b'/', Some(b'A')).is_none());
    }

    /// Every config `new` accepts must encode to US-ASCII; `append_to_string` relies on it for its `unsafe`.
    #[test]
    fn every_valid_config_encodes_as_ascii() {
        let data: Vec<u8> = (0u8..=255).collect();
        for v63 in 0u8..=255 {
            for v64 in 0u8..=255 {
                for padding in [None, Some(b'=')] {
                    if let Some(encoder) = Base64Encoder::new(v63, v64, padding) {
                        let encoded: String = encoder.encode_as_string(&data).unwrap();
                        assert!(encoded.is_ascii(), "v63={v63:#04X} v64={v64:#04X}");
                    }
                }
            }
        }
    }

    #[test]
    fn encode() {
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
            (b"\xF3\xDF\xBF", "89-_"),
            (b"", ""),
            (b"\x00", "AA"),
            (b"\xFF", "_w"),
            (b"\x00\x00", "AAA"),
            (b"\xFF\xFF", "__8"),
            (b"\x00\x00\x00", "AAAA"),
            (b"\xFF\xFF\xFF", "____"),
            (b"\x00\x00\x00\x00", "AAAAAA"),
            (b"\xFF\xFF\xFF\xFF", "_____w"),
            (b"\x00\x00\x00\x00\x00", "AAAAAAA"),
            (b"\xFF\xFF\xFF\xFF\xFF", "______8"),
            (b"\x00\x00\x00\x00\x00\x00", "AAAAAAAA"),
            (b"\xFF\xFF\xFF\xFF\xFF\xFF", "________"),
        ];
        let encoder: Base64Encoder = Base64Encoder::url_safe_encoder();
        test_string_encoder(&encoder, test_cases);
    }

    #[test]
    fn encode_default() {
        let test_cases: &[(&[u8], &str)] = &[
            (b"", ""),
            (b"\x00\x10\x83", "ABCD"),
            (b"\xF3\xDF\xBF", "89+/"),
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
        test_string_encoder(&encoder, test_cases);
    }
}
