use crate::base_64::Base64Encoder;
use crate::base_64::decode;
use crate::base_64::decode::DecodingTable;
use crate::{Error, Validator};
use std::fmt::{Debug, Formatter};

/// Responsible for validating base-64 encoded data.
///
/// # Trailing Bits
/// The unused bits of the last encoded char must be zero. A 2-char last block holds 8 bits of data in 12 and a
/// 3-char last block holds 16 in 18, so the extra bits are checked. This rejects encodings like `AB` that decode
/// cleanly but that no encoder would produce.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Base64Validator {
    decoding_table: DecodingTable,
    padding: Option<u8>,
    require_padding: bool,
}

impl Base64Validator {
    //! Construction

    /// Creates a base-64 validator.
    ///
    /// Returns `None` if the encoding config is invalid.
    pub fn new(v63: u8, v64: u8, padding: Option<u8>, require_padding: bool) -> Option<Self> {
        if Base64Encoder::is_valid_config(v63, v64, padding)
            && (padding.is_some() || !require_padding)
        {
            Some(Self {
                decoding_table: DecodingTable::get_decoding_table(v63, v64),
                padding,
                require_padding,
            })
        } else {
            None
        }
    }
}

impl Default for Base64Validator {
    fn default() -> Self {
        Self {
            decoding_table: DecodingTable::default(),
            padding: Base64Encoder::DEFAULT_PADDING,
            require_padding: false,
        }
    }
}

impl Base64Validator {
    //! Validation

    /// Checks if the last two bytes are valid. Padding is invalid.
    ///
    /// # Safety
    /// The `data` length must be exactly 2.
    #[inline(always)]
    unsafe fn is_valid_2_not_padded(decoding_table: &[u8; 256], data: &[u8]) -> bool {
        debug_assert_eq!(data.len(), 2);

        let a: usize = unsafe { *data.get_unchecked(0) } as usize;
        let b: usize = unsafe { *data.get_unchecked(1) } as usize;

        let a: u32 = unsafe { *decoding_table.get_unchecked(a) } as u32;
        let b: u32 = unsafe { *decoding_table.get_unchecked(b) } as u32;

        let bits: u32 = (a << 8) | b;
        (bits & 0x808F) == 0
    }

    /// Checks if the last three bytes are valid. Padding is invalid.
    ///
    /// # Safety
    /// The `data` length must be exactly 3.
    #[inline(always)]
    unsafe fn is_valid_3_not_padded(decoding_table: &[u8; 256], data: &[u8]) -> bool {
        debug_assert_eq!(data.len(), 3);

        let a: usize = unsafe { *data.get_unchecked(0) } as usize;
        let b: usize = unsafe { *data.get_unchecked(1) } as usize;
        let c: usize = unsafe { *data.get_unchecked(2) } as usize;

        let a: u32 = unsafe { *decoding_table.get_unchecked(a) } as u32;
        let b: u32 = unsafe { *decoding_table.get_unchecked(b) } as u32;
        let c: u32 = unsafe { *decoding_table.get_unchecked(c) } as u32;

        let bits: u32 = (a << 16) | (b << 8) | c;
        (bits & 0x808083) == 0
    }

    /// Checks if the last four bytes are valid. Padding is invalid.
    ///
    /// # Safety
    /// The `data` length must be exactly 4.
    #[inline(always)]
    unsafe fn is_valid_4_not_padded(decoding_table: &[u8; 256], data: &[u8]) -> bool {
        debug_assert_eq!(data.len(), 4);

        let a: usize = unsafe { *data.get_unchecked(0) } as usize;
        let b: usize = unsafe { *data.get_unchecked(1) } as usize;
        let c: usize = unsafe { *data.get_unchecked(2) } as usize;
        let d: usize = unsafe { *data.get_unchecked(3) } as usize;

        let a: u32 = unsafe { *decoding_table.get_unchecked(a) } as u32;
        let b: u32 = unsafe { *decoding_table.get_unchecked(b) } as u32;
        let c: u32 = unsafe { *decoding_table.get_unchecked(c) } as u32;
        let d: u32 = unsafe { *decoding_table.get_unchecked(d) } as u32;

        let bits: u32 = (a << 24) | (b << 16) | (c << 8) | d;
        (bits & 0x80808080) == 0
    }

    /// Checks if the last block is valid.
    ///
    /// # Safety
    /// The `data` length must be at most 4.
    #[inline(always)]
    unsafe fn is_valid_block_last(
        decoding_table: &[u8; 256],
        padding: Option<u8>,
        require_padding: bool,
        data: &[u8],
    ) -> bool {
        debug_assert!(data.len() <= 4);

        match data.len() {
            1 => false,
            2 => !require_padding && unsafe { Self::is_valid_2_not_padded(decoding_table, data) },
            3 => {
                !require_padding
                    && unsafe { Self::is_valid_3_not_padded(decoding_table, &data[..3]) }
            }
            4 => {
                if let Some(padding) = padding {
                    let d: u8 = unsafe { *data.get_unchecked(3) };
                    if d == padding {
                        let c: u8 = unsafe { *data.get_unchecked(2) };
                        if c == padding {
                            unsafe { Self::is_valid_2_not_padded(decoding_table, &data[..2]) }
                        } else {
                            unsafe { Self::is_valid_3_not_padded(decoding_table, &data[..3]) }
                        }
                    } else {
                        unsafe { Self::is_valid_4_not_padded(decoding_table, data) }
                    }
                } else {
                    unsafe { Self::is_valid_4_not_padded(decoding_table, data) }
                }
            }
            _ => unreachable!(),
        }
    }
}

impl Validator for Base64Validator {
    fn is_valid(&self, data: &[u8]) -> Result<bool, Error> {
        if data.is_empty() {
            Ok(true)
        } else {
            let (full_blocks, last_block): (&[u8], &[u8]) = decode::split_last_block(data);
            let decoding_table: &[u8; 256] = self.decoding_table.table();

            let last_block_valid: bool = unsafe {
                Self::is_valid_block_last(
                    decoding_table,
                    self.padding,
                    self.require_padding,
                    last_block,
                )
            };
            if last_block_valid {
                for block in full_blocks.as_chunks::<4>().0 {
                    let block_valid: bool =
                        unsafe { Self::is_valid_4_not_padded(decoding_table, block) };
                    if !block_valid {
                        return Ok(false);
                    }
                }
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }
}

impl Debug for Base64Validator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (v63, v64) = self.decoding_table.values();
        f.debug_struct("Base64Validator")
            .field("v63", &(v63 as char))
            .field("v64", &(v64 as char))
            .field("padding", &self.padding.map(|p| p as char))
            .field("require_padding", &self.require_padding)
            .finish()
    }
}

#[cfg(test)]
#[cfg(feature = "dev")]
mod tests {
    use crate::base_64::base_64_validator::Base64Validator;
    use crate::test::test_validator;

    #[test]
    fn new() {
        // `require_padding` cannot be satisfied without a padding char
        assert!(Base64Validator::new(b'-', b'_', None, true).is_none());
        assert!(Base64Validator::new(b'-', b'_', None, false).is_some());
        assert!(Base64Validator::new(b'+', b'/', Some(b'='), true).is_some());

        // the encoding config itself is still validated
        assert!(Base64Validator::new(b'+', b'+', Some(b'='), false).is_none());
        assert!(Base64Validator::new(b'+', b'/', Some(b'+'), false).is_none());
    }

    #[test]
    fn is_valid_require_padding() {
        let test_cases: &[(&str, bool)] = &[
            ("", true),
            ("AA", false),
            ("AAA", false),
            ("AAAA", true),
            ("AA==", true),
            ("AAA=", true),
            ("AAAAAA", false),
            ("AAAAAA==", true),
        ];
        let validator: Base64Validator =
            Base64Validator::new(b'+', b'/', Some(b'='), true).unwrap();
        test_validator(&validator, test_cases);
    }

    #[test]
    fn is_valid_url_safe() {
        let test_cases: &[(&str, bool)] = &[
            ("", true),
            ("AA", true),
            ("AAA", true),
            ("AAAA", true),
            ("89-_", true),
            ("89+/", false),
            ("AA==", false),
        ];
        let validator: Base64Validator = Base64Validator::new(b'-', b'_', None, false).unwrap();
        test_validator(&validator, test_cases);
    }

    /// The unused bits of the last encoded char must be zero. (see the `Trailing Bits` docs)
    #[test]
    fn is_valid_trailing_bits() {
        let test_cases: &[(&str, bool)] = &[
            ("AA", true),   // 'A' = 0b000000
            ("AQ", true),   // 'Q' = 0b010000
            ("AB", false),  // 'B' = 0b000001
            ("AP", false),  // 'P' = 0b001111
            ("AAA", true),  // 'A' = 0b000000
            ("AAE", true),  // 'E' = 0b000100
            ("AAB", false), // 'B' = 0b000001
            ("AA==", true),
            ("AB==", false),
        ];
        test_validator(&Base64Validator::default(), test_cases);
    }

    #[test]
    fn is_valid_default() {
        let test_cases: &[(&str, bool)] = &[
            ("ABCD", true),
            ("EFGH", true),
            ("IJKL", true),
            ("MNOP", true),
            ("QRST", true),
            ("UVWX", true),
            ("YZab", true),
            ("cdef", true),
            ("ghij", true),
            ("klmn", true),
            ("opqr", true),
            ("stuv", true),
            ("wxyz", true),
            ("0123", true),
            ("4567", true),
            ("89+/", true),
            ("....", false),
            ("::::", false),
            ("@@@@", false),
            ("[[[[", false),
            ("````", false),
            ("{{{{", false),
            ("****", false),
            (",,,,", false),
            ("", true),
            ("A", false),
            ("AA", true),
            ("AAA", true),
            ("AAAA", true),
            ("AA=", false),
            ("AA==", true),
            ("AA=A", false),
            ("A===", false),
            ("AA==AAAA", false),
            ("AAAAAAAA", true),
            ("AAAAAA==", true),
            ("AAAAA~==", false),
        ];
        let validator: Base64Validator = Base64Validator::default();
        test_validator(&validator, test_cases);
    }
}
