use crate::{Error, Validator};

/// Responsible for validating hexadecimal encoded data.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct HexValidator {
    allow_lowercase: bool,
    allow_uppercase: bool,
}

impl HexValidator {
    //! Special Validators

    /// The case-insensitive hex validator.
    pub const CASELESS: Self = Self {
        allow_lowercase: true,
        allow_uppercase: true,
    };

    /// The lowercase-only hex validator.
    pub const LOWER_ONLY: Self = Self {
        allow_lowercase: true,
        allow_uppercase: false,
    };

    /// The uppercase-only hex validator.
    pub const UPPER_ONLY: Self = Self {
        allow_lowercase: false,
        allow_uppercase: true,
    };
}

impl Default for HexValidator {
    fn default() -> Self {
        Self::CASELESS
    }
}

impl HexValidator {
    //! Validation

    /// Checks if `b` is valid.
    #[inline(always)]
    pub const fn is_valid_byte(&self, b: u8) -> bool {
        b.is_ascii_digit()
            || (self.allow_lowercase && matches!(b, b'a'..=b'f'))
            || (self.allow_uppercase && matches!(b, b'A'..=b'F'))
    }

    /// Checks if `c` is valid.
    #[inline(always)]
    pub const fn is_valid_char(&self, c: char) -> bool {
        let c: u32 = c as u32;
        c <= (b'f' as u32) && self.is_valid_byte((c & 0xFF) as u8)
    }
}

impl Validator for HexValidator {
    fn is_valid(&self, data: &[u8]) -> Result<bool, Error> {
        Ok(data.len().is_multiple_of(2)
            && match (self.allow_lowercase, self.allow_uppercase) {
                (true, true) => data.iter().all(|c| c.is_ascii_hexdigit()),
                (true, false) => data
                    .iter()
                    .all(|c| c.is_ascii_digit() || matches!(c, b'a'..=b'f')),
                (false, true) => data
                    .iter()
                    .all(|c| c.is_ascii_digit() || matches!(c, b'A'..=b'F')),
                _ => unreachable!(),
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::hex::HexValidator;
    use crate::Validator;
    use std::error::Error;

    #[test]
    fn is_valid() -> Result<(), Box<dyn Error>> {
        let test_cases: &[(&str, bool, bool)] = &[
            ("", true, true),
            ("0", false, false),
            ("0/", false, false),
            ("0:", false, false),
            ("0@", false, false),
            ("0G", false, false),
            ("0`", false, false),
            ("0g", false, false),
            ("01", true, true),
            ("012", false, false),
            ("0123", true, true),
            ("01234", false, false),
            ("012345", true, true),
            ("0123456", false, false),
            ("01234567", true, true),
            ("012345678", false, false),
            ("0123456789", true, true),
            ("abcdef", true, false),
            ("ABCDEF", false, true),
        ];

        // todo -- validator testing
        for (input, lower_only, upper_only) in test_cases {
            let result: bool = HexValidator::LOWER_ONLY.is_valid(input.as_bytes())?;
            assert_eq!(result, *lower_only, "input={}", *input);

            let result: bool = HexValidator::UPPER_ONLY.is_valid(input.as_bytes())?;
            assert_eq!(result, *upper_only, "input={}", *input);

            let result: bool = HexValidator::CASELESS.is_valid(input.as_bytes())?;
            assert_eq!(result, *upper_only || *lower_only, "input={}", *input);
        }

        Ok(())
    }
}
