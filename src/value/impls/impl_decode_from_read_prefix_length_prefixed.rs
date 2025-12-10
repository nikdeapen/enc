/// Implements of the `DecodeFromReadPrefix` trait for the `target_type` by using the
/// `DecodeFromRead::decode_from_read_length_prefixed_with_first_byte` function on the same
/// `targeT_type`.
#[macro_export]
macro_rules! impl_decode_from_read_prefix_length_prefixed {
    ($target_type:ty) => {
        impl $crate::DecodeFromReadPrefix for $target_type {
            fn decode_from_read_prefix_with_first_byte<R>(
                r: &mut R,
                first: u8,
            ) -> Result<Self, $crate::Error>
            where
                R: std::io::Read,
            {
                use $crate::DecodeFromRead;

                Self::decode_from_read_length_prefixed_with_first_byte(r, first)
            }
        }
    };
}
