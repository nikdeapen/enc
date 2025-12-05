use crate::var_int::impl_var_int::{VarInt128, VarInt16, VarInt32, VarInt64, VarIntSize};
use crate::{impl_encode_to_write_stack_buf, Error};

macro_rules! impl_var_int_encode {
    ($target_type:ty, $unsigned_type:ty, $bit_size:expr) => {
        impl $crate::EncodedLen for $target_type {
            fn encoded_len(&self) -> Result<usize, Error> {
                Ok(($bit_size - (self.value | 1).leading_zeros()).div_ceil(7) as usize)
            }
        }

        impl $crate::EncodeToSlice for $target_type {
            unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
                let mut t: usize = 0;
                let mut v: $unsigned_type = self.value;
                unsafe {
                    for _ in 0..(Self::MAX_ENCODED_LEN - 1) {
                        let last_seven: u8 = (v & 0x7F) as u8;
                        v >>= 7;
                        if v == 0 {
                            *target.get_unchecked_mut(t) = last_seven;
                            return Ok(t + 1);
                        } else {
                            *target.get_unchecked_mut(t) = last_seven | 0x80;
                            t += 1;
                        }
                    }
                    *target.get_unchecked_mut(t) = v as u8;
                }
                Ok(t + 1)
            }
        }

        impl_encode_to_write_stack_buf!($target_type, ($bit_size / 8) as usize);
    };
}

impl_var_int_encode!(VarInt16, u16, u16::BITS);
impl_var_int_encode!(VarInt32, u32, u32::BITS);
impl_var_int_encode!(VarInt64, u64, u64::BITS);
impl_var_int_encode!(VarInt128, u128, u128::BITS);
impl_var_int_encode!(VarIntSize, usize, usize::BITS);
