use crate::var_int::impl_var_int::{VarInt128, VarInt16, VarInt32, VarInt64, VarIntSize};

macro_rules! impl_var_int_decode {
    ($target_type:ident, $unsigned_type:ty) => {
        impl $crate::DecodeFromReadPrefix for $target_type {
            fn decode_from_read_prefix_with_first_byte<R>(
                r: &mut R,
                first: u8,
            ) -> Result<Self, $crate::Error>
            where
                R: std::io::Read,
            {
                let mut result: $unsigned_type = (first & 0x7F) as $unsigned_type;
                if first & 0x80 == 0 {
                    Ok(Self::from(result))
                } else {
                    let mut shift: usize = 7;
                    for _ in 0..(Self::MAX_ENCODED_LEN - 2) {
                        let b: u8 = $crate::read_single_byte(r)?;
                        if b & 0x80 == 0 {
                            result |= (b as $unsigned_type) << shift;
                            return Ok(Self::from(result));
                        } else {
                            result |= ((b & 0x7F) as $unsigned_type) << shift;
                            shift += 7;
                        }
                    }
                    let b: u8 = $crate::read_single_byte(r)?;
                    if b & Self::LAST_BYTE_MASK != 0 {
                        Err($crate::Error::InvalidEncodedData { reason: None }.into())
                    } else {
                        result |= (b as $unsigned_type) << (7 * (Self::MAX_ENCODED_LEN - 1));
                        Ok(result.into())
                    }
                }
            }
        }
    };
}

impl_var_int_decode!(VarInt16, u16);
impl_var_int_decode!(VarInt32, u32);
impl_var_int_decode!(VarInt64, u64);
impl_var_int_decode!(VarInt128, u128);
impl_var_int_decode!(VarIntSize, usize);
