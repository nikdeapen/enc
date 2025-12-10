#[macro_export]
macro_rules! impl_decode_from_read_by_prefix {
    ($target_type:ty) => {
        impl $crate::DecodeFromRead for $target_type {
            fn decode_from_read<R>(r: &mut R) -> Result<Self, $crate::Error>
            where
                R: std::io::Read,
            {
                use $crate::DecodeFromReadPrefix;

                let value: Self = Self::decode_from_read_prefix(r)?;
                debug_assert!($crate::read_optional_byte(r)?.is_none());
                Ok(value)
            }
        }
    };
}
