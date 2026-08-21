use crate::hex::HexValidator;
use crate::percent::SpecialSet;
use crate::percent::prefix_is_encoded::prefix_is_encoded;
use crate::{Error, Validator};

/// Responsible for validating percent encoded data.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
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

/// Creates a percent validator with the case-insensitive hex validator.
impl<S: Into<SpecialSet>> From<S> for PercentValidator {
    fn from(encoding_not_needed: S) -> Self {
        Self::new(HexValidator::CASELESS, encoding_not_needed.into())
    }
}

impl Validator for PercentValidator {
    fn is_valid(&self, data: &[u8]) -> Result<bool, Error> {
        let mut i: usize = 0;
        while i < data.len() {
            let c: u8 = data[i];
            if !c.is_ascii_alphanumeric() {
                if c == b'%' {
                    if !prefix_is_encoded(self.hex_validator, &data[i..]) {
                        return Ok(false);
                    }
                    i += 3;
                    continue;
                } else if !self.encoding_not_needed.contains(c) {
                    return Ok(false);
                }
            }
            i += 1;
        }
        Ok(true)
    }
}

#[cfg(test)]
#[cfg(feature = "dev")]
mod tests {
    use crate::percent::PercentValidator;
    use crate::test::test_validator;

    #[test]
    fn is_valid() {
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
        let validator: PercentValidator = "+-.".into();
        test_validator(&validator, test_cases);
    }
}
