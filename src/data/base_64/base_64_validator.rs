use crate::base_64::decode::decoding_table::DecodingTable;
use crate::base_64::Base64Encoder;
use crate::{Error, Validator};

/// Responsible for validating base-64 encoded data.
#[derive(Clone, Debug)]
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
    pub fn from(v63: u8, v64: u8, padding: Option<u8>, require_padding: bool) -> Option<Self> {
        if Base64Encoder::is_valid_config(v63, v64, padding) {
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

        let bits: u32 = unsafe {
            let a: usize = *data.get_unchecked(0) as usize;
            let b: usize = *data.get_unchecked(1) as usize;

            let a: u32 = *decoding_table.get_unchecked(a) as u32;
            let b: u32 = *decoding_table.get_unchecked(b) as u32;

            (a << 8) | b
        };

        (bits & 0x808F) == 0
    }

    /// Checks if the last three bytes are valid. Padding is invalid.
    ///
    /// # Safety
    /// The `data` length must be exactly 3.
    #[inline(always)]
    unsafe fn is_valid_3_not_padded(decoding_table: &[u8; 256], data: &[u8]) -> bool {
        debug_assert_eq!(data.len(), 3);

        let bits: u32 = unsafe {
            let a: usize = *data.get_unchecked(0) as usize;
            let b: usize = *data.get_unchecked(1) as usize;
            let c: usize = *data.get_unchecked(2) as usize;

            let a: u32 = *decoding_table.get_unchecked(a) as u32;
            let b: u32 = *decoding_table.get_unchecked(b) as u32;
            let c: u32 = *decoding_table.get_unchecked(c) as u32;

            (a << 16) | (b << 8) | c
        };

        (bits & 0x808083) == 0
    }

    /// Checks if the last four bytes are valid. Padding is invalid.
    ///
    /// # Safety
    /// The `data` length must be exactly 4.
    #[inline(always)]
    unsafe fn is_valid_4_not_padded(decoding_table: &[u8; 256], data: &[u8]) -> bool {
        debug_assert_eq!(data.len(), 4);

        let bits: u32 = unsafe {
            let a: usize = *data.get_unchecked(0) as usize;
            let b: usize = *data.get_unchecked(1) as usize;
            let c: usize = *data.get_unchecked(2) as usize;
            let d: usize = *data.get_unchecked(3) as usize;

            let a: u32 = *decoding_table.get_unchecked(a) as u32;
            let b: u32 = *decoding_table.get_unchecked(b) as u32;
            let c: u32 = *decoding_table.get_unchecked(c) as u32;
            let d: u32 = *decoding_table.get_unchecked(d) as u32;

            (a << 24) | (b << 16) | (c << 8) | d
        };

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

        unsafe {
            match data.len() {
                1 => false,
                2 => !require_padding && Self::is_valid_2_not_padded(decoding_table, data),
                3 => !require_padding && Self::is_valid_3_not_padded(decoding_table, &data[..3]),
                4 => {
                    if let Some(padding) = padding {
                        let d: u8 = *data.get_unchecked(3);
                        if d == padding {
                            let c: u8 = *data.get_unchecked(2);
                            if c == padding {
                                Self::is_valid_2_not_padded(decoding_table, &data[..2])
                            } else {
                                Self::is_valid_3_not_padded(decoding_table, &data[..3])
                            }
                        } else {
                            Self::is_valid_4_not_padded(decoding_table, data)
                        }
                    } else {
                        Self::is_valid_4_not_padded(decoding_table, data)
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

impl Validator for Base64Validator {
    fn is_valid(&self, data: &[u8]) -> Result<bool, Error> {
        let len: usize = data.len();
        if len == 0 {
            Ok(true)
        } else {
            let rem: usize = len % 4;
            let last_chunk_index: usize = data.len() - if rem == 0 { 4 } else { rem };
            let decoding_table: &[u8; 256] = self.decoding_table.decoding_table();

            let last_block_valid: bool = unsafe {
                Self::is_valid_block_last(
                    decoding_table,
                    self.padding,
                    self.require_padding,
                    &data[last_chunk_index..],
                )
            };
            if last_block_valid {
                for d in 0..(last_chunk_index / 4) {
                    let block_valid: bool = unsafe {
                        Self::is_valid_4_not_padded(decoding_table, &data[d * 4..(d * 4 + 4)])
                    };
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

#[cfg(test)]
mod tests {
    use crate::base_64::base_64_validator::Base64Validator;
    use crate::Validator;
    use std::error::Error;

    #[test]
    fn is_valid_default() -> Result<(), Box<dyn Error>> {
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

        // todo -- validator testing
        let validator: Base64Validator = Base64Validator::default();
        for (input, expected) in test_cases {
            let result: bool = validator.is_valid(input.as_bytes())?;
            assert_eq!(result, *expected, "input={}", *input);
        }

        Ok(())
    }
}
