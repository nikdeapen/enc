use std::fmt::{Display, Formatter};
use std::io;

use crate::var_int::VarInt64;
use crate::Error::InvalidEncodedData;
use crate::{read_single_byte, Error, StreamError};
use crate::{DecodeFromReadPrefix, EncodeToSlice, EncodeToWrite, EncodedLen};

/// A variable-length encoded `u128` value.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct VarInt128 {
    pub value: u128,
}

impl From<u128> for VarInt128 {
    fn from(value: u128) -> Self {
        Self { value }
    }
}

impl From<&u128> for VarInt128 {
    fn from(value: &u128) -> Self {
        Self::from(*value)
    }
}

impl VarInt128 {
    //! Constants

    /// The maximum length of an encoded `u128` value. (19)
    pub const MAX_ENCODED_LEN: usize = ((u128::BITS + 6) / 7) as usize;

    /// The last decoded byte mask. (used to ensure a decoded value does not overflow)
    const LAST_DECODING_BYTE_MASK: u8 = 0xFF << (u128::BITS % 7);
}

impl VarInt128 {
    //! Zig-Zag

    /// Creates a `VarInt128` from the `i128` value.
    pub fn from_i128(value: i128) -> Self {
        Self::from(((value << 1) ^ (value >> 127)) as u128)
    }

    /// Converts the `u128` value to a `VarInt128`.
    pub fn to_i128(&self) -> i128 {
        ((self.value >> 1) as i128) ^ (-((self.value & 1) as i128))
    }
}

impl EncodedLen for VarInt128 {
    fn encoded_len(&self) -> Result<usize, Error> {
        let encoded_len: usize = if self.value == 0 {
            1
        } else {
            let bits: u32 = u128::BITS - self.value.leading_zeros();
            ((bits + 6) / 7) as usize
        };
        Ok(encoded_len)
    }
}

impl EncodeToSlice for VarInt128 {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        let mut t: usize = 0;
        let mut v: u128 = self.value;
        while v > (u64::MAX as u128) {
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
        Ok(t + VarInt64::from(v as u64).encode_to_slice_unchecked(&mut target[t..])?)
    }
}

impl EncodeToWrite for VarInt128 {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, StreamError>
    where
        W: io::Write,
    {
        let mut buffer: [u8; Self::MAX_ENCODED_LEN] = [0u8; Self::MAX_ENCODED_LEN];
        let encoded_len: usize = unsafe { self.encode_to_slice_unchecked(&mut buffer)? };
        w.write_all(&mut buffer[..encoded_len])?;
        Ok(encoded_len)
    }
}

impl DecodeFromReadPrefix for VarInt128 {
    fn decode_from_read_prefix_with_first_byte<R>(first: u8, r: &mut R) -> Result<Self, StreamError>
    where
        R: io::Read,
    {
        let mut result: u128 = (first & 0x7F) as u128;
        if first & 0x80 == 0 {
            Ok(Self::from(result))
        } else {
            let mut shift: usize = 7;
            for _ in 0..(Self::MAX_ENCODED_LEN - 2) {
                let b: u8 = read_single_byte(r)?;
                if b & 0x80 == 0 {
                    result |= (b as u128) << shift;
                    return Ok(Self::from(result));
                } else {
                    result |= ((b & 0x7F) as u128) << shift;
                    shift += 7;
                }
            }
            let b: u8 = read_single_byte(r)?;
            if b & Self::LAST_DECODING_BYTE_MASK != 0 {
                Err(InvalidEncodedData.into())
            } else {
                result |= (b as u128) << (7 * (Self::MAX_ENCODED_LEN - 1));
                Ok(result.into())
            }
        }
    }
}

impl Display for VarInt128 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::io::Cursor;

    use crate::var_int::VarInt128;
    use crate::{
        DecodeFromReadPrefix, EncodeToSlice, EncodeToWrite, EncodedLen, Error, StreamError,
    };

    #[test]
    fn max_encoded_len() -> Result<(), Error> {
        assert_eq!(
            VarInt128::MAX_ENCODED_LEN,
            VarInt128::from(u128::MAX).encoded_len()?
        );
        Ok(())
    }

    const TEST_CASES: &[(u128, &[u8])] = &[
        (0x00, b"\x00"),       // 0 bits
        (0x01, b"\x01"),       // 1 bit
        (0x7F, b"\x7F"),       // highest one byte value
        (0x80, b"\x80\x01"),   // lowest two byte value
        (0x3FFF, b"\xFF\x7F"), // highest two byte value
        (
            u128::MAX,
            b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x03",
        ),
    ];

    #[test]
    fn encode() -> Result<(), io::Error> {
        for (value, expected) in TEST_CASES {
            let value: VarInt128 = VarInt128::from(value);

            let encoded_len: usize = value.encoded_len()?;
            assert_eq!(encoded_len, expected.len(), "value={}", value);

            let encoded: Vec<u8> = value.encode_as_vec()?;
            assert_eq!(encoded, *expected, "value={}", value);

            let mut output: Cursor<Vec<u8>> = Cursor::default();
            value.encode_to_write(&mut output)?;
            assert_eq!(output.into_inner(), *expected);
        }
        Ok(())
    }

    #[test]
    fn decode() -> Result<(), io::Error> {
        for (expected, input) in TEST_CASES {
            let mut cursor: Cursor<Vec<u8>> = Cursor::new(input.to_vec());
            let result: Result<VarInt128, StreamError> =
                VarInt128::decode_from_read_prefix(&mut cursor);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().value, *expected);

            let mut extra: Vec<u8> = input.to_vec();
            extra.push(0xFF);
            let mut cursor: Cursor<Vec<u8>> = Cursor::new(extra);
            let result: Result<VarInt128, StreamError> =
                VarInt128::decode_from_read_prefix(&mut cursor);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().value, *expected);
        }
        Ok(())
    }
}
