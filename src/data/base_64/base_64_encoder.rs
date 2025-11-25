use crate::base_64::encode;
use crate::base_64::encode::EncodingTable;
use crate::Error::InsufficientTargetSpace;
use crate::{data, Encoder, Error, StringEncoder};

/// Responsible for encoding data in the base-64 format.
#[derive(Clone, Debug)]
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
            let table: &[u8; 64] = self.table.encoding_table();
            let div: usize = data.len() / 3;
            let rem: usize = data.len() % 3;
            let mut d: usize = 0;
            let mut t: usize = 0;
            for _ in 0..div {
                unsafe { encode::encode_block(table, &data[d..], &mut target[t..]) };
                d += 3;
                t += 4;
            }
            match rem {
                0 => {}
                1 => {
                    t += unsafe {
                        encode::encode_last_block_1(
                            table,
                            self.padding,
                            &data[d..],
                            &mut target[t..],
                        )
                    }
                }
                2 => {
                    t += unsafe {
                        encode::encode_last_block_2(
                            table,
                            self.padding,
                            &data[d..],
                            &mut target[t..],
                        )
                    }
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
        unsafe { data::util::append_to_string_unchecked(self, data, target) }
    }
}

#[cfg(test)]
mod tests {
    use crate::base_64::Base64Encoder;
    use crate::StringEncoder;

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
            (b"\xF3\xDF\xBF", "89-_"),
            (b"", ""),
            (b"\x00", "AA=="),
            (b"\xFF", "_w=="),
            (b"\x00\x00", "AAA="),
            (b"\xFF\xFF", "__8="),
            (b"\x00\x00\x00", "AAAA"),
            (b"\xFF\xFF\xFF", "____"),
            (b"\x00\x00\x00\x00", "AAAAAA=="),
            (b"\xFF\xFF\xFF\xFF", "_____w=="),
            (b"\x00\x00\x00\x00\x00", "AAAAAAA="),
            (b"\xFF\xFF\xFF\xFF\xFF", "______8="),
            (b"\x00\x00\x00\x00\x00\x00", "AAAAAAAA"),
            (b"\xFF\xFF\xFF\xFF\xFF\xFF", "________"),
        ];

        // todo -- encoder testing
        let encoder: Base64Encoder = Base64Encoder::url_safe_encoder();
        for (data, expected) in test_cases {
            let result: String = encoder.encode_as_string(data)?;
            assert_eq!(result, *expected, "data={data:?}");
        }

        Ok(())
    }
}
