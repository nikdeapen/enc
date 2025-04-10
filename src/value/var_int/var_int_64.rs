use std::fmt::{Display, Formatter};
use std::io;
use std::io::ErrorKind::InvalidData;

use crate::read_single_byte;
use crate::Error::InvalidEncodedData;
use crate::{DecodeFromReadPrefix, EncodeToSlice, EncodeToWrite, EncodedLen, Error};

/// A variable-length encoded `u64` value.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct VarInt64 {
    pub value: u64,
}

impl From<u64> for VarInt64 {
    fn from(value: u64) -> Self {
        Self { value }
    }
}

impl From<&u64> for VarInt64 {
    fn from(value: &u64) -> Self {
        Self::from(*value)
    }
}

impl VarInt64 {
    //! Constants

    /// The maximum length of an encoded `u64` value. (10)
    pub const MAX_ENCODED_LEN: usize = ((u64::BITS + 6) / 7) as usize;

    /// The last decoded byte mask. (used to ensure a decoded value does not overflow)
    const LAST_DECODING_BYTE_MASK: u8 = 0xFF << (u64::BITS % 7);

    /// Converts a signed i64 to an unsigned u64 using zigzag encoding.
    ///
    /// Zigzag encoding maps signed integers to unsigned integers so that numbers
    /// with a small absolute value have a small encoded value too.
    ///
    /// # Examples
    ///
    /// ```
    /// use enc::var_int::VarInt64;
    /// assert_eq!(VarInt64::zigzag_encode(0), 0);
    /// assert_eq!(VarInt64::zigzag_encode(-1), 1);
    /// assert_eq!(VarInt64::zigzag_encode(1), 2);
    /// assert_eq!(VarInt64::zigzag_encode(-2), 3);
    /// ```
    pub fn zigzag_encode(value: i64) -> u64 {
        ((value << 1) ^ (value >> 63)) as u64
    }

    /// Converts an unsigned u64 to a signed i64 using zigzag decoding.
    ///
    /// # Examples
    ///
    /// ```
    /// use enc::var_int::VarInt64;
    /// assert_eq!(VarInt64::zigzag_decode(0), 0);
    /// assert_eq!(VarInt64::zigzag_decode(1), -1);
    /// assert_eq!(VarInt64::zigzag_decode(2), 1);
    /// assert_eq!(VarInt64::zigzag_decode(3), -2);
    /// ```
    pub fn zigzag_decode(value: u64) -> i64 {
        ((value >> 1) as i64) ^ (-((value & 1) as i64))
    }

    /// Creates a VarInt64 from a signed i64 value using zigzag encoding.
    pub fn from_i64(value: i64) -> Self {
        Self {
            value: Self::zigzag_encode(value),
        }
    }

    /// Converts this VarInt64 to a signed i64 value using zigzag decoding.
    pub fn to_i64(&self) -> i64 {
        Self::zigzag_decode(self.value)
    }
}

impl EncodedLen for VarInt64 {
    fn encoded_len(&self) -> Result<usize, Error> {
        let encoded_len: usize = if self.value == 0 {
            1
        } else {
            let bits: u32 = u64::BITS - self.value.leading_zeros();
            ((bits + 6) / 7) as usize
        };
        Ok(encoded_len)
    }
}

impl EncodeToSlice for VarInt64 {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        let mut t: usize = 0;
        let mut v: u64 = self.value;
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
        Ok(t + 1)
    }
}

impl EncodeToWrite for VarInt64 {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, io::Error>
    where
        W: io::Write,
    {
        let mut buffer: [u8; Self::MAX_ENCODED_LEN] = [0u8; Self::MAX_ENCODED_LEN];
        let encoded_len: usize = unsafe { self.encode_to_slice_unchecked(&mut buffer)? };
        w.write_all(&mut buffer[..encoded_len])?;
        Ok(encoded_len)
    }
}

impl DecodeFromReadPrefix for VarInt64 {
    fn decode_from_read_prefix_with_first_byte<R>(first: u8, r: &mut R) -> Result<Self, io::Error>
    where
        R: io::Read,
    {
        let mut result: u64 = (first & 0x7F) as u64;
        if first & 0x80 == 0 {
            Ok(Self::from(result))
        } else {
            let mut shift: usize = 7;
            for _ in 0..(Self::MAX_ENCODED_LEN - 2) {
                let b: u8 = read_single_byte(r)?;
                if b & 0x80 == 0 {
                    result |= (b as u64) << shift;
                    return Ok(Self::from(result));
                } else {
                    result |= ((b & 0x7F) as u64) << shift;
                    shift += 7;
                }
            }
            let b: u8 = read_single_byte(r)?;
            if b & Self::LAST_DECODING_BYTE_MASK != 0 {
                Err(io::Error::new(InvalidData, InvalidEncodedData))
            } else {
                result |= (b as u64) << (7 * (Self::MAX_ENCODED_LEN - 1));
                Ok(result.into())
            }
        }
    }
}

impl Display for VarInt64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::io::Cursor;

    use crate::var_int::VarInt64;
    use crate::{DecodeFromReadPrefix, EncodeToSlice, EncodeToWrite, EncodedLen, Error};

    #[test]
    fn max_encoded_len() -> Result<(), Error> {
        assert_eq!(
            VarInt64::MAX_ENCODED_LEN,
            VarInt64::from(u64::MAX).encoded_len()?
        );
        Ok(())
    }

    const TEST_CASES: &[(u64, &[u8])] = &[
        (0x00, b"\x00"),       // 0 bits
        (0x01, b"\x01"),       // 1 bit
        (0x7F, b"\x7F"),       // highest one byte value
        (0x80, b"\x80\x01"),   // lowest two byte value
        (0x3FFF, b"\xFF\x7F"), // highest two byte value
        (u64::MAX, b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01"),
    ];

    #[test]
    fn encode() -> Result<(), io::Error> {
        for (value, expected) in TEST_CASES {
            let value: VarInt64 = VarInt64::from(value);

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
            let result: Result<VarInt64, io::Error> =
                VarInt64::decode_from_read_prefix(&mut cursor);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().value, *expected);

            let mut extra: Vec<u8> = input.to_vec();
            extra.push(0xFF);
            let mut cursor: Cursor<Vec<u8>> = Cursor::new(extra);
            let result: Result<VarInt64, io::Error> =
                VarInt64::decode_from_read_prefix(&mut cursor);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().value, *expected);
        }
        Ok(())
    }

    #[test]
    fn zigzag_encoding() {
        // Test cases: (signed, unsigned)
        let test_cases = [
            (0i64, 0u64),
            (1i64, 2u64),
            (-1i64, 1u64),
            (2i64, 4u64),
            (-2i64, 3u64),
            (i64::MAX, (i64::MAX as u64) * 2),
            (i64::MIN, u64::MAX),
        ];

        for (signed, unsigned) in test_cases {
            // Test encoding
            assert_eq!(VarInt64::zigzag_encode(signed), unsigned);

            // Test decoding
            assert_eq!(VarInt64::zigzag_decode(unsigned), signed);

            // Test convenience methods
            let var_int = VarInt64::from_i64(signed);
            assert_eq!(var_int.value, unsigned);
            assert_eq!(var_int.to_i64(), signed);
        }
    }
}
