/// Responsible for checking if encoded data is valid.
pub trait Validator {
    /// Checks if the encoded data is valid.
    fn is_valid(&self, data: &[u8]) -> bool;
}
