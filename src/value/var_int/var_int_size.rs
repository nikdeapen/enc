use std::fmt::{Display, Formatter};
use std::io;
use std::io::ErrorKind::InvalidData;

use crate::Error::InvalidEncodedData;
use crate::{read_single_byte, Error};
use crate::{DecodeFromReadPrefix, EncodeToSlice, EncodeToWrite, EncodedLen};

/// A variable-length encoded `usize` value.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct VarIntSize {
    pub value: usize,
}

impl From<usize> for VarIntSize {
    fn from(value: usize) -> Self {
        Self { value }
    }
}

impl From<&usize> for VarIntSize {
    fn from(value: &usize) -> Self {
        Self::from(*value)
    }
}

impl VarIntSize {
    //! Constants

    /// The maximum length of an encoded `usize` value.
    pub const MAX_ENCODED_LEN: usize = ((usize::BITS + 6) / 7) as usize;

    /// The last decoded byte mask. (used to ensure a decoded value does not overflow)
    const LAST_DECODING_BYTE_MASK: u8 = 0xFF << (usize::BITS % 7);
}

impl VarIntSize {
    //! Zig-Zag

    /// Creates a `VarIntSize` from the `isize` value.
    pub fn from_isize(value: isize) -> Self {
        Self::from(((value << 1) ^ (value >> (isize::BITS - 1))) as usize)
    }

    /// Converts the `usize` value to a `VarIntSize`.
    pub fn to_isize(&self) -> isize {
        ((self.value >> 1) as isize) ^ (-((self.value & 1) as isize))
    }
}

impl EncodedLen for VarIntSize {
    fn encoded_len(&self) -> Result<usize, Error> {
        let encoded_len: usize = if self.value == 0 {
            1
        } else {
            let bits: u32 = usize::BITS - self.value.leading_zeros();
            ((bits + 6) / 7) as usize
        };
        Ok(encoded_len)
    }
}

impl EncodeToSlice for VarIntSize {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        let mut t: usize = 0;
        let mut v: usize = self.value;
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

impl EncodeToWrite for VarIntSize {
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

impl DecodeFromReadPrefix for VarIntSize {
    fn decode_from_read_prefix_with_first_byte<R>(first: u8, r: &mut R) -> Result<Self, io::Error>
    where
        R: io::Read,
    {
        let mut result: usize = (first & 0x7F) as usize;
        if first & 0x80 == 0 {
            Ok(Self::from(result))
        } else {
            let mut shift: usize = 7;
            for _ in 0..(Self::MAX_ENCODED_LEN - 2) {
                let b: u8 = read_single_byte(r)?;
                if b & 0x80 == 0 {
                    result |= (b as usize) << shift;
                    return Ok(Self::from(result));
                } else {
                    result |= ((b & 0x7F) as usize) << shift;
                    shift += 7;
                }
            }
            let b: u8 = read_single_byte(r)?;
            if b & Self::LAST_DECODING_BYTE_MASK != 0 {
                Err(io::Error::new(InvalidData, InvalidEncodedData))
            } else {
                result |= (b as usize) << (7 * (Self::MAX_ENCODED_LEN - 1));
                Ok(result.into())
            }
        }
    }
}

impl Display for VarIntSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use std::{io, usize};

    use crate::var_int::VarIntSize;
    use crate::{DecodeFromReadPrefix, EncodeToSlice, EncodeToWrite, EncodedLen, Error};

    #[test]
    fn max_encoded_len() -> Result<(), Error> {
        assert_eq!(
            VarIntSize::MAX_ENCODED_LEN,
            VarIntSize::from(usize::MAX).encoded_len()?
        );
        Ok(())
    }

    const TEST_CASES: &[(usize, &[u8])] = &[
        (0x00, b"\x00"),       // 0 bits
        (0x01, b"\x01"),       // 1 bit
        (0x7F, b"\x7F"),       // highest one byte value
        (0x80, b"\x80\x01"),   // lowest two byte value
        (0x3FFF, b"\xFF\x7F"), // highest two byte value
        #[cfg(target_pointer_width = "32")]
        (usize::MAX, b"\xFF\xFF\xFF\xFF\x0F"),
        #[cfg(target_pointer_width = "64")]
        (usize::MAX, b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01"),
    ];

    #[test]
    fn encode() -> Result<(), io::Error> {
        for (value, expected) in TEST_CASES {
            let value: VarIntSize = VarIntSize::from(value);

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
            let result: Result<VarIntSize, io::Error> =
                VarIntSize::decode_from_read_prefix(&mut cursor);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().value, *expected);

            let mut extra: Vec<u8> = input.to_vec();
            extra.push(0xFF);
            let mut cursor: Cursor<Vec<u8>> = Cursor::new(extra);
            let result: Result<VarIntSize, io::Error> =
                VarIntSize::decode_from_read_prefix(&mut cursor);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().value, *expected);
        }
        Ok(())
    }
}
