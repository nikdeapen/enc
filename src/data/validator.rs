use crate::Error;

/// Responsible for validating encoded data.
pub trait Validator {
    /// Checks if the encoded `data` is valid.
    fn is_valid(&self, data: &[u8]) -> Result<bool, Error>;
}
