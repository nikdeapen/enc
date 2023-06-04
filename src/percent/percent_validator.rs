use crate::hex::HexValidator;
use crate::percent::SpecialSet;
use crate::Validator;

/// Responsible for validating hex encoded data.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct PercentValidator {
    encoding_not_needed: SpecialSet,
    hex_validator: HexValidator,
}

impl<S: Into<SpecialSet>> From<S> for PercentValidator {
    fn from(encoding_not_needed: S) -> Self {
        Self {
            encoding_not_needed: encoding_not_needed.into(),
            hex_validator: HexValidator::default(),
        }
    }
}

impl PercentValidator {
    //! Validation

    /// Checks if the byte needs to be encoded.
    pub fn needs_encoding(&self, b: u8) -> bool {
        !(b.is_ascii_alphanumeric() || self.encoding_not_needed.contains(b))
    }
}

impl Validator for PercentValidator {
    fn is_valid(&self, data: &[u8]) -> bool {
        let mut d: usize = 0;
        while d < data.len() {
            if data[d] == b'%' {
                if d + 1 == data.len() || d + 2 == data.len() {
                    return false;
                } else if !self.hex_validator.is_valid_byte(data[d + 1]) {
                    return false;
                } else if !self.hex_validator.is_valid_byte(data[d + 2]) {
                    return false;
                }
                d += 3;
            } else if self.needs_encoding(data[d]) {
                return false;
            } else {
                d += 1;
            }
        }
        true
    }
}
