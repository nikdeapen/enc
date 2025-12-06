use std::fmt::{Display, Formatter};

macro_rules! impl_var_int {
    ($target_type:ident, $unsigned_type:ty, $signed_type:ty, $bit_len:expr) => {
        /// A variable-length encoded `$unsigned_type` value.
        #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
        pub struct $target_type {
            pub(in crate::var_int) value: $unsigned_type,
        }

        impl From<$unsigned_type> for $target_type {
            fn from(value: $unsigned_type) -> Self {
                Self { value }
            }
        }

        impl From<&$unsigned_type> for $target_type {
            fn from(value: &$unsigned_type) -> Self {
                Self::from(*value)
            }
        }

        impl From<$signed_type> for $target_type {
            fn from(value: $signed_type) -> Self {
                Self::from_zig_zag(value)
            }
        }

        impl From<&$signed_type> for $target_type {
            fn from(value: &$signed_type) -> Self {
                Self::from(*value)
            }
        }

        impl $target_type {
            //! Constants

            /// The maximum length of a var-int encoded `$unsigned_type` value.
            pub const MAX_ENCODED_LEN: usize = $bit_len.div_ceil(7) as usize;

            /// The last decoded byte mask. (used to detect integer overflow while decoding)
            pub(in crate::value::var_int) const LAST_BYTE_MASK: u8 = 0xFF << ($bit_len % 7);
        }

        impl $target_type {
            //! Properties

            /// Gets the value.
            pub fn value(&self) -> $unsigned_type {
                self.value
            }
        }

        impl $target_type {
            //! Zig-Zag

            /// Creates a `$target_type` from the `$signed_type` value using zig-zag encoding.
            pub fn from_zig_zag(value: $signed_type) -> Self {
                Self::from(((value << 1) ^ (value >> ($bit_len - 1))) as $unsigned_type)
            }

            /// Converts the `$target_type` to an `$signed_type` value using zig-zag encoding.
            pub fn to_zig_zag(&self) -> $signed_type {
                ((self.value >> 1) as $signed_type) ^ (-((self.value & 1) as $signed_type))
            }
        }

        impl Display for $target_type {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.value)
            }
        }
    };
}

impl_var_int!(VarInt16, u16, i16, u16::BITS);
impl_var_int!(VarInt32, u32, i32, u32::BITS);
impl_var_int!(VarInt64, u64, i64, u64::BITS);
impl_var_int!(VarInt128, u128, i128, u128::BITS);
impl_var_int!(VarIntSize, usize, isize, usize::BITS);

#[cfg(test)]
mod tests {
    use crate::var_int::{VarInt128, VarInt16, VarInt32, VarInt64, VarIntSize};
    use crate::EncodedLen;
    use std::error::Error;

    #[test]
    fn max_encoded_len() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            VarInt16::MAX_ENCODED_LEN,
            VarInt16::from(u16::MAX).encoded_len()?
        );
        assert_eq!(
            VarInt32::MAX_ENCODED_LEN,
            VarInt32::from(u32::MAX).encoded_len()?
        );
        assert_eq!(
            VarInt64::MAX_ENCODED_LEN,
            VarInt64::from(u64::MAX).encoded_len()?
        );
        assert_eq!(
            VarInt128::MAX_ENCODED_LEN,
            VarInt128::from(u128::MAX).encoded_len()?
        );
        assert_eq!(
            VarIntSize::MAX_ENCODED_LEN,
            VarIntSize::from(usize::MAX).encoded_len()?
        );
        Ok(())
    }

    #[test]
    fn from_zig_zag() {
        let test_cases: &[(i32, u32)] = &[
            (0, 0),
            (-1, 1),
            (1, 2),
            (-2, 3),
            (2, 4),
            (-3, 5),
            (3, 6),
            (-4, 7),
            (4, 8),
            (-5, 9),
            (5, 10),
        ];

        for (value, expected) in test_cases {
            let result: u32 = VarInt32::from_zig_zag(*value).value;
            assert_eq!(result, *expected);
        }
    }

    #[test]
    fn to_zig_zag() {
        for i in -100i32..100i32 {
            let result: i16 = VarInt16::from_zig_zag(i as i16).to_zig_zag();
            assert_eq!(result, i as i16);

            let result: i32 = VarInt32::from_zig_zag(i).to_zig_zag();
            assert_eq!(result, i);

            let result: i64 = VarInt64::from_zig_zag(i as i64).to_zig_zag();
            assert_eq!(result, i as i64);

            let result: i128 = VarInt128::from_zig_zag(i as i128).to_zig_zag();
            assert_eq!(result, i as i128);

            let result: isize = VarIntSize::from_zig_zag(i as isize).to_zig_zag();
            assert_eq!(result, i as isize);
        }
    }

    #[test]
    fn zig_zag_max() {
        let result: i16 = VarInt16::from_zig_zag(i16::MAX).to_zig_zag();
        assert_eq!(result, i16::MAX);

        let result: i32 = VarInt32::from_zig_zag(i32::MAX).to_zig_zag();
        assert_eq!(result, i32::MAX);

        let result: i64 = VarInt64::from_zig_zag(i64::MAX).to_zig_zag();
        assert_eq!(result, i64::MAX);

        let result: i128 = VarInt128::from_zig_zag(i128::MAX).to_zig_zag();
        assert_eq!(result, i128::MAX);

        let result: isize = VarIntSize::from_zig_zag(isize::MAX).to_zig_zag();
        assert_eq!(result, isize::MAX);
    }
}
