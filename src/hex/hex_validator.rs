use crate::hex::HexDecoder;
use crate::Validator;

/// Responsible for validating hex encoded data.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct HexValidator {
    allow_lower: bool,
    allow_upper: bool,
}

impl HexValidator {
    //! Special Validators

    /// The case-insensitive hex validator.
    pub const CASELESS: Self = Self {
        allow_lower: true,
        allow_upper: true,
    };

    /// The lowercase-only hex validator.
    pub const LOWER_ONLY: Self = Self {
        allow_lower: true,
        allow_upper: false,
    };

    /// The uppercase-only hex validator.
    pub const UPPER_ONLY: Self = Self {
        allow_lower: false,
        allow_upper: true,
    };
}

impl Default for HexValidator {
    fn default() -> Self {
        Self::CASELESS
    }
}

impl HexValidator {
    //! Validation

    /// Checks if the hex byte is valid.
    #[inline(always)]
    pub const fn is_valid_byte(&self, b: u8) -> bool {
        HexDecoder::TABLE[b as usize] != HexDecoder::INV
            && match (self.allow_lower, self.allow_upper) {
                (true, true) => true,
                (true, false) => !b.is_ascii_uppercase(),
                (false, true) => !b.is_ascii_lowercase(),
                _ => unreachable!(),
            }
    }

    /// Checks if the hex char is valid.
    #[inline(always)]
    pub const fn is_valid_char(&self, c: char) -> bool {
        c.is_ascii() && self.is_valid_byte(c as u8)
    }
}

impl Validator for HexValidator {
    fn is_valid(&self, data: &[u8]) -> bool {
        data.len() % 2 == 0
            && match (self.allow_lower, self.allow_upper) {
                (true, true) => data
                    .iter()
                    .all(|b| HexDecoder::TABLE[*b as usize] != HexDecoder::INV),
                (true, false) => data.iter().all(|b| {
                    HexDecoder::TABLE[*b as usize] != HexDecoder::INV && !b.is_ascii_uppercase()
                }),
                (false, true) => data.iter().all(|b| {
                    HexDecoder::TABLE[*b as usize] != HexDecoder::INV && !b.is_ascii_lowercase()
                }),
                _ => unreachable!(),
            }
    }
}
