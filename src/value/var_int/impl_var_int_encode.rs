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

        impl_encode_to_write_stack_buf!($target_type, Self::MAX_ENCODED_LEN);
    };
}

impl_var_int_encode!(VarInt16, u16, u16::BITS);
impl_var_int_encode!(VarInt32, u32, u32::BITS);
impl_var_int_encode!(VarInt64, u64, u64::BITS);
impl_var_int_encode!(VarInt128, u128, u128::BITS);
impl_var_int_encode!(VarIntSize, usize, usize::BITS);

#[cfg(test)]
#[cfg(feature = "test")]
mod tests {
    use crate::test::{test_decode_from_read_prefix, test_encode};
    use crate::var_int::{VarInt128, VarInt16, VarInt32, VarInt64, VarIntSize};

    #[test]
    fn var_int_16() {
        let test_cases: &[(u16, &[u8])] = &[
            (0x00, b"\x00"),             // 0 bits
            (0x01, b"\x01"),             // 1 bit
            (0x7F, b"\x7F"),             // highest one byte value
            (0x80, b"\x80\x01"),         // lowest two-byte value
            (0x3FFF, b"\xFF\x7F"),       // highest two-byte value
            (u16::MAX, b"\xFF\xFF\x03"), // max
        ];

        for (value, encoded) in test_cases {
            let value: VarInt16 = VarInt16::from(value);
            test_encode(&value, encoded);
            test_decode_from_read_prefix(encoded, &value, false);
        }
    }

    #[test]
    fn var_int_32() {
        let test_cases: &[(u32, &[u8])] = &[
            (0x00, b"\x00"),                     // 0 bits
            (0x01, b"\x01"),                     // 1 bit
            (0x7F, b"\x7F"),                     // highest one byte value
            (0x80, b"\x80\x01"),                 // lowest two-byte value
            (0x3FFF, b"\xFF\x7F"),               // highest two-byte value
            (u32::MAX, b"\xFF\xFF\xFF\xFF\x0F"), // max
        ];

        for (value, encoded) in test_cases {
            let value: VarInt32 = VarInt32::from(value);
            test_encode(&value, encoded);
            test_decode_from_read_prefix(encoded, &value, false);
        }
    }

    #[test]
    fn var_int_64() {
        let test_cases: &[(u64, &[u8])] = &[
            (0x00, b"\x00"),       // 0 bits
            (0x01, b"\x01"),       // 1 bit
            (0x7F, b"\x7F"),       // highest one byte value
            (0x80, b"\x80\x01"),   // lowest two-byte value
            (0x3FFF, b"\xFF\x7F"), // highest two-byte value
            (u64::MAX, b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01"),
        ];

        for (value, encoded) in test_cases {
            let value: VarInt64 = VarInt64::from(value);
            test_encode(&value, encoded);
            test_decode_from_read_prefix(encoded, &value, false);
        }
    }

    #[test]
    fn var_int_128() {
        let test_cases: &[(u128, &[u8])] = &[
            (0x00, b"\x00"),       // 0 bits
            (0x01, b"\x01"),       // 1 bit
            (0x7F, b"\x7F"),       // highest one byte value
            (0x80, b"\x80\x01"),   // lowest two-byte value
            (0x3FFF, b"\xFF\x7F"), // highest two-byte value
            (
                u128::MAX,
                b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x03",
            ),
        ];

        for (value, encoded) in test_cases {
            let value: VarInt128 = VarInt128::from(value);
            test_encode(&value, encoded);
            test_decode_from_read_prefix(encoded, &value, false);
        }
    }

    #[test]
    fn var_int_size() {
        let test_cases: &[(usize, &[u8])] = &[
            (0x00, b"\x00"),       // 0 bits
            (0x01, b"\x01"),       // 1 bit
            (0x7F, b"\x7F"),       // highest one byte value
            (0x80, b"\x80\x01"),   // lowest two-byte value
            (0x3FFF, b"\xFF\x7F"), // highest two-byte value
            #[cfg(target_pointer_width = "32")]
            (usize::MAX, b"\xFF\xFF\xFF\xFF\x0F"),
            #[cfg(target_pointer_width = "64")]
            (usize::MAX, b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01"),
        ];

        for (value, encoded) in test_cases {
            let value: VarIntSize = VarIntSize::from(value);
            test_encode(&value, encoded);
            test_decode_from_read_prefix(encoded, &value, false);
        }
    }
}
