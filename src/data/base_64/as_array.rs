/// Converts the `slice` to an array reference of the exact same length.
///
/// # Panics
/// Panics if the `slice` length is not exactly `N`.
#[inline(always)]
pub fn as_array<const N: usize>(slice: &[u8]) -> &[u8; N] {
    slice.try_into().expect("slice length must be exactly N")
}

/// Converts the `slice` to a mutable array reference of the exact same length.
///
/// # Panics
/// Panics if the `slice` length is not exactly `N`.
#[inline(always)]
pub fn as_array_mut<const N: usize>(slice: &mut [u8]) -> &mut [u8; N] {
    slice.try_into().expect("slice length must be exactly N")
}
