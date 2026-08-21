use crate::hex::HexValidator;

/// Checks if the prefix of `data` is a valid, encoded byte.
///
/// # Note
/// The `data` must not be empty and the first byte must be the `%` symbol.
#[inline(always)]
pub fn prefix_is_encoded(hex_validator: HexValidator, data: &[u8]) -> bool {
    debug_assert!(!data.is_empty());
    debug_assert_eq!(data[0], b'%');

    data.len() >= 3 && hex_validator.is_valid_byte(data[1]) && hex_validator.is_valid_byte(data[2])
}
