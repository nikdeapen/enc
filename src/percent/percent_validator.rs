use crate::hex::HexValidator;
use crate::percent::SpecialSet;
use crate::Validator;

/// Responsible for validating percent encoded data.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct PercentValidator {
    hex_validator: HexValidator,
    encoding_not_needed: SpecialSet,
}

impl<S: Into<SpecialSet>> From<S> for PercentValidator {
    fn from(encoding_not_needed: S) -> Self {
        Self {
            hex_validator: HexValidator::default(),
            encoding_not_needed: encoding_not_needed.into(),
        }
    }
}

impl Validator for PercentValidator {
    fn is_valid(&self, data: &[u8]) -> bool {
        for (i, c) in data.iter().enumerate() {
            if !c.is_ascii_alphanumeric() && !c.is_ascii_digit() {
                if *c == b'%' {
                    if i + 2 >= data.len() {
                        return false;
                    } else if !self.hex_validator.is_valid_byte(data[i + 1])
                        || !self.hex_validator.is_valid_byte(data[i + 2])
                    {
                        return false;
                    }
                } else if !self.encoding_not_needed.contains(*c) {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::percent::PercentValidator;
    use crate::Validator;

    #[test]
    fn is_valid() {
        let test_cases: &[(&str, bool)] = &[
            ("", true),
            ("%", false),
            ("%0", false),
            ("%0x", false),
            ("%x0", false),
            ("%00", true),
            ("%aA", true),
            ("%Aa", true),
            ("+-.", true),
            ("!", false),
            ("~", false),
            (" ", false),
            ("你好", false),
        ];
        let validator: PercentValidator = "+-.".into();
        for (data, expected) in test_cases {
            let result: bool = validator.is_valid(data.as_bytes());
            assert_eq!(result, *expected, "data={}", *data);
        }
    }
}
