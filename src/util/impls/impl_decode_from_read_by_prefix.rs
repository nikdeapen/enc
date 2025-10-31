/// Implements the `DecodeFromRead` trait for the `target_type` by delegating to the
/// `DecodeFromReadPrefix::decode_from_read_prefix_with_first_byte` .
#[macro_export]
macro_rules! impl_decode_from_read_by_prefix {
    ($target_type:ty) => {
        impl enc::DecodeFromRead for $target_type {
            fn decode_from_read<R>(r: &mut R) -> Result<Self, enc::Error>
            where
                R: std::io::Read,
            {
                use enc::DecodeFromReadPrefix;

                let first: u8 = enc::read_single_byte(r)?;
                let value: Self = Self::decode_from_read_prefix_with_first_byte(r, first)?;
                debug_assert!(enc::read_optional_byte(r)?.is_none());
                Ok(value)
            }
        }
    };
}
