use crate::var_int::impl_var_int::{VarInt16, VarInt32, VarInt64, VarInt128, VarIntSize};

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
                        Err($crate::Error::InvalidEncodedData { reason: None })
                    } else {
                        result |= (b as $unsigned_type) << (7 * (Self::MAX_ENCODED_LEN - 1));
                        Ok(result.into())
                    }
                }
            }
        }

        $crate::impl_decode_from_read_by_prefix!($target_type);
    };
}

impl_var_int_decode!(VarInt16, u16);
impl_var_int_decode!(VarInt32, u32);
impl_var_int_decode!(VarInt64, u64);
impl_var_int_decode!(VarInt128, u128);
impl_var_int_decode!(VarIntSize, usize);

#[cfg(test)]
#[cfg(feature = "dev")]
mod tests {
    use crate::DecodeFromReadPrefix;
    use crate::var_int::{VarInt16, VarInt32, VarInt64, VarInt128, VarIntSize};
    use std::io::Cursor;

    fn decodes_as_overflow<T: DecodeFromReadPrefix>(encoded: &[u8]) -> bool {
        T::decode_from_read_prefix(&mut Cursor::new(encoded)).is_err()
    }

    #[test]
    fn var_int_16_overflow() {
        // last byte mask = 0xFC; last byte must only use bits 0-1
        assert!(decodes_as_overflow::<VarInt16>(b"\xFF\xFF\x04")); // 0x04 & 0xFC != 0
        assert!(decodes_as_overflow::<VarInt16>(b"\xFF\xFF\xFF")); // 0xFF & 0xFC != 0
    }

    #[test]
    fn var_int_32_overflow() {
        // last byte mask = 0xF0; last byte must only use bits 0-3
        assert!(decodes_as_overflow::<VarInt32>(b"\xFF\xFF\xFF\xFF\x10")); // 0x10 & 0xF0 != 0
        assert!(decodes_as_overflow::<VarInt32>(b"\xFF\xFF\xFF\xFF\xFF")); // 0xFF & 0xF0 != 0
    }

    #[test]
    fn var_int_64_overflow() {
        // last byte mask = 0xFE; last byte must only use bit 0
        assert!(decodes_as_overflow::<VarInt64>(
            b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02"
        )); // 0x02 & 0xFE != 0
        assert!(decodes_as_overflow::<VarInt64>(
            b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF"
        )); // 0xFF & 0xFE != 0
    }

    #[test]
    fn var_int_128_overflow() {
        // last byte mask = 0xFC; last byte must only use bits 0-1
        let mut encoded: Vec<u8> = vec![0xFF; 18];
        encoded.push(0x04); // 0x04 & 0xFC != 0
        assert!(decodes_as_overflow::<VarInt128>(&encoded));

        let mut encoded: Vec<u8> = vec![0xFF; 18];
        encoded.push(0xFF);
        assert!(decodes_as_overflow::<VarInt128>(&encoded));
    }

    #[test]
    fn var_int_size_overflow() {
        #[cfg(target_pointer_width = "32")]
        {
            // last byte mask = 0xF0; last byte must only use bits 0-3
            assert!(decodes_as_overflow::<VarIntSize>(b"\xFF\xFF\xFF\xFF\x10"));
            assert!(decodes_as_overflow::<VarIntSize>(b"\xFF\xFF\xFF\xFF\xFF"));
        }
        #[cfg(target_pointer_width = "64")]
        {
            // last byte mask = 0xFE; last byte must only use bit 0
            assert!(decodes_as_overflow::<VarIntSize>(
                b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02"
            ));
            assert!(decodes_as_overflow::<VarIntSize>(
                b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF"
            ));
        }
    }
}
