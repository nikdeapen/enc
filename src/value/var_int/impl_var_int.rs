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

        impl $target_type {
            //! Constants

            /// The maximum length of a var-int encoded `$unsigned_type` value.
            pub const MAX_ENCODED_LEN: usize = $bit_len.div_ceil(7) as usize;

            /// The last decoded byte mask. (used to detect integer overflow while decoding)
            pub(in crate::value::var_int) const LAST_DECODING_BYTE_MASK: u8 =
                0xFF << ($bit_len % 7);
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
