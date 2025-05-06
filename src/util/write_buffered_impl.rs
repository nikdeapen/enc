/// Implements `EncodeToWrite` for a type that also implements `EncodeToSlice` where the encoding
/// has a small maximum encoded length. The data is buffered on the stack before it is written out.
#[macro_export]
macro_rules! write_stack_buf_impl {
    ($target_type:ty, $max_len:expr) => {
        impl $crate::EncodeToWrite for $target_type {
            fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, $crate::StreamError>
            where
                W: std::io::Write,
            {
                use $crate::EncodeToSlice;

                let mut buffer: [u8; $max_len] = [0u8; $max_len];
                let encoded_len: usize = unsafe { self.encode_to_slice_unchecked(&mut buffer)? };
                w.write_all(&buffer[..encoded_len])?;
                Ok(encoded_len)
            }
        }
    };
}
