/// Implements the `EncodeToWrite` trait for the `target_type`.
///
/// The `target_type` must also implement `EncodeToSlice` and have a relatively small maximum
/// encoded length (that will fit on the stack). The `encode_to_slice_unchecked` function is used
/// to encode the value to the stack before it is flushed to the `Write`.
#[macro_export]
macro_rules! impl_encode_to_write_stack_buf {
    ($target_type:ty, $max_encoded_len:expr) => {
        impl $crate::EncodeToWrite for $target_type {
            fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, $crate::Error>
            where
                W: std::io::Write,
            {
                use $crate::EncodeToSlice;

                let mut buffer: [u8; $max_encoded_len] = [0u8; $max_encoded_len];
                let encoded_len: usize = unsafe { self.encode_to_slice_unchecked(&mut buffer)? };
                w.write_all(&buffer[..encoded_len])?;
                Ok(encoded_len)
            }
        }
    };
}
