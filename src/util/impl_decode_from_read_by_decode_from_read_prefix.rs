/// Implements the `DecodeFromRead` trait for trait where it is equivalent to the
/// `DecodeFromReadPrefix` implementation. Also adds a stream exhaustion debug assertion.
#[macro_export]
macro_rules! impl_decode_from_read_by_decode_from_read_prefix {
    ($target_type:ty) => {
        impl $crate::DecodeFromRead for $target_type {
            fn decode_from_read<R>(r: &mut R) -> Result<Self, StreamError>
            where
                R: Read;
            {
                use $crate::DecodeFromReadPrefix;

                let value: Self = Self::decode_from_read_prefix(r)?;
                debug_assert_eq!(read_optional_byte(r)?, None);
                Ok(value)
            }
        }
    };
}
