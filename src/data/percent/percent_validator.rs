use crate::hex::HexValidator;
use crate::percent::SpecialSet;
use crate::{Error, Validator};

/// Responsible for validating percent encoded data.
#[derive(Copy, Clone, Debug, Default)]
pub struct PercentValidator {
    hex_validator: HexValidator,
    encoding_not_needed: SpecialSet,
}

impl PercentValidator {
    //! Construction

    /// Creates a new percent validator.
    pub const fn new(hex_validator: HexValidator, encoding_not_needed: SpecialSet) -> Self {
        Self {
            hex_validator,
            encoding_not_needed,
        }
    }
}

impl<S: Into<SpecialSet>> From<S> for PercentValidator {
    fn from(encoding_not_needed: S) -> Self {
        Self::new(HexValidator::CASELESS, encoding_not_needed.into())
    }
}

impl Validator for PercentValidator {
    fn is_valid(&self, data: &[u8]) -> Result<bool, Error> {
        for (i, c) in data.iter().enumerate() {
            if !c.is_ascii_alphanumeric() && !c.is_ascii_digit() {
                if *c == b'%' {
                    if i + 2 >= data.len()
                        || !self.hex_validator.is_valid_byte(data[i + 1])
                        || !self.hex_validator.is_valid_byte(data[i + 2])
                    {
                        return Ok(false);
                    }
                } else if !self.encoding_not_needed.contains(*c) {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use crate::percent::PercentValidator;
    use crate::Validator;
    use std::error::Error;

    #[test]
    fn is_valid() -> Result<(), Box<dyn Error>> {
        let test_cases: &[(&str, bool)] = &[
            ("", true),
            ("%", false),
            ("%0", false),
            ("%0x", false),
            ("%x0", false),
            ("%09", true),
            ("%90", true),
            ("%aA", true),
            ("%Aa", true),
            ("+-.", true),
            ("!", false),
            ("~", false),
            (" ", false),
            ("你好", false),
        ];

        // todo - validator testing
        let validator: PercentValidator = "+-.".into();
        for (data, expected) in test_cases {
            let result: bool = validator.is_valid(data.as_bytes())?;
            assert_eq!(result, *expected, "data={}", *data);
        }

        Ok(())
    }
}
