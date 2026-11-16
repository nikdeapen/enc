/// Implements the `DecodeFromSlice` trait for the `target_type` by delegating it to the
/// `DecodeFromReadPrefix::decode_from_read_prefix` function on the same `target_type`.
///
/// # Note
/// - The decoding fails if the slice has data left over after the encoded value.
#[macro_export]
macro_rules! impl_decode_from_slice_by_prefix {
    ($target_type:ty) => {
        impl $crate::DecodeFromSlice for $target_type {
            fn decode_from_slice(data: &[u8]) -> Result<Self, $crate::Error> {
                use $crate::DecodeFromReadPrefix;

                let mut remaining: &[u8] = data;
                let value: Self = Self::decode_from_read_prefix(&mut remaining)?;
                if remaining.is_empty() {
                    Ok(value)
                } else {
                    Err($crate::Error::invalid_encoded_data())
                }
            }
        }
    };
}
