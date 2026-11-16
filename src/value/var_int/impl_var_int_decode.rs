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
                        Err($crate::Error::invalid_encoded_data())
                    } else {
                        result |= (b as $unsigned_type) << (7 * (Self::MAX_ENCODED_LEN - 1));
                        Ok(result.into())
                    }
                }
            }
        }

        $crate::impl_decode_from_read_by_prefix!($target_type);
        $crate::impl_decode_from_slice_by_prefix!($target_type);
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
    use crate::test::hex;
    use crate::var_int::{VarInt16, VarInt32, VarInt64, VarInt128, VarIntSize};
    use crate::{DecodeFromReadPrefix, DecodeFromSlice};
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

    #[test]
    fn var_int_16_decode_from_slice() {
        let test_cases: &[(&[u8], Option<u16>)] = &[
            (b"\x00", Some(0x00)),
            (b"\x80\x01", Some(0x80)),
            (b"\xFF\xFF\x03", Some(u16::MAX)),
            (b"", None),                 // empty
            (b"\x80", None),             // truncated
            (b"\x00\x00", None),         // trailing byte
            (b"\xFF\xFF\x03\x00", None), // trailing byte
            (b"\xFF\xFF\x04", None),     // overflow
        ];

        for (encoded, expected) in test_cases {
            let result: Option<u16> = VarInt16::decode_from_slice(encoded)
                .ok()
                .map(VarInt16::value);
            assert_eq!(result, *expected, "encoded={}", hex(encoded));
        }
    }

    #[test]
    fn var_int_32_decode_from_slice() {
        let test_cases: &[(&[u8], Option<u32>)] = &[
            (b"\x00", Some(0x00)),
            (b"\x80\x01", Some(0x80)),
            (b"\xFF\xFF\xFF\xFF\x0F", Some(u32::MAX)),
            (b"", None),                         // empty
            (b"\x80", None),                     // truncated
            (b"\x00\x00", None),                 // trailing byte
            (b"\xFF\xFF\xFF\xFF\x0F\x00", None), // trailing byte
            (b"\xFF\xFF\xFF\xFF\x10", None),     // overflow
        ];

        for (encoded, expected) in test_cases {
            let result: Option<u32> = VarInt32::decode_from_slice(encoded)
                .ok()
                .map(VarInt32::value);
            assert_eq!(result, *expected, "encoded={}", hex(encoded));
        }
    }

    #[test]
    fn var_int_64_decode_from_slice() {
        let test_cases: &[(&[u8], Option<u64>)] = &[
            (b"\x00", Some(0x00)),
            (b"\x80\x01", Some(0x80)),
            (b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01", Some(u64::MAX)),
            (b"", None),                                             // empty
            (b"\x80", None),                                         // truncated
            (b"\x00\x00", None),                                     // trailing byte
            (b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x00", None), // trailing byte
            (b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02", None),     // overflow
        ];

        for (encoded, expected) in test_cases {
            let result: Option<u64> = VarInt64::decode_from_slice(encoded)
                .ok()
                .map(VarInt64::value);
            assert_eq!(result, *expected, "encoded={}", hex(encoded));
        }
    }

    #[test]
    fn var_int_128_decode_from_slice() {
        let test_cases: &[(&[u8], Option<u128>)] = &[
            (b"\x00", Some(0x00)),
            (b"\x80\x01", Some(0x80)),
            (
                b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x03",
                Some(u128::MAX),
            ),
            (b"", None),         // empty
            (b"\x80", None),     // truncated
            (b"\x00\x00", None), // trailing byte
            (
                b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x03\x00",
                None,
            ), // trailing byte
            (
                b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04",
                None,
            ), // overflow
        ];

        for (encoded, expected) in test_cases {
            let result: Option<u128> = VarInt128::decode_from_slice(encoded)
                .ok()
                .map(VarInt128::value);
            assert_eq!(result, *expected, "encoded={}", hex(encoded));
        }
    }

    #[test]
    fn var_int_size_decode_from_slice() {
        let test_cases: &[(&[u8], Option<usize>)] = &[
            (b"\x00", Some(0x00)),
            (b"\x80\x01", Some(0x80)),
            #[cfg(target_pointer_width = "32")]
            (b"\xFF\xFF\xFF\xFF\x0F", Some(usize::MAX)),
            #[cfg(target_pointer_width = "64")]
            (
                b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01",
                Some(usize::MAX),
            ),
            (b"", None),         // empty
            (b"\x80", None),     // truncated
            (b"\x00\x00", None), // trailing byte
            #[cfg(target_pointer_width = "32")]
            (b"\xFF\xFF\xFF\xFF\x0F\x00", None), // trailing byte
            #[cfg(target_pointer_width = "64")]
            (b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x00", None), // trailing byte
            #[cfg(target_pointer_width = "32")]
            (b"\xFF\xFF\xFF\xFF\x10", None), // overflow
            #[cfg(target_pointer_width = "64")]
            (b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02", None), // overflow
        ];

        for (encoded, expected) in test_cases {
            let result: Option<usize> = VarIntSize::decode_from_slice(encoded)
                .ok()
                .map(VarIntSize::value);
            assert_eq!(result, *expected, "encoded={}", hex(encoded));
        }
    }
}
