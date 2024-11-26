use std::fmt::{Display, Formatter};
use std::io;
use std::io::ErrorKind::InvalidData;

use crate::var_int::VarInt32;
use crate::Error::InvalidEncodedData;
use crate::{DecodeFromReadPrefix, EncodeToSlice, EncodeToWrite, EncodedLen, Error};

/// A variable-length encoded `u16` value.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct VarInt16 {
    pub value: u16,
}

impl From<u16> for VarInt16 {
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl From<&u16> for VarInt16 {
    fn from(value: &u16) -> Self {
        Self::from(*value)
    }
}

impl VarInt16 {
    //! Constants

    /// The maximum length of an encoded `u16` value. (3)
    pub const MAX_ENCODED_LEN: usize = ((u16::BITS + 6) / 7) as usize;
}

impl VarInt16 {
    //! ZigZag

    /// Creates a `VarInt16` from the signed `value` using zigzag encoding.
    pub fn from_zigzag(value: i16) -> Self {
        let value: u16 = value as u16;
        let value: u16 = (value << 1) ^ (value >> 15);
        Self::from(value)
    }

    /// Creates a `VarInt16` from the signed `value` using zigzag encoding.
    pub fn to_zigzag(&self) -> i16 {
        ((self.value >> 1) as i16) ^ -((self.value & 1) as i16)
    }
}

impl EncodedLen for VarInt16 {
    fn encoded_len(&self) -> Result<usize, Error> {
        VarInt32::from(self.value as u32).encoded_len()
    }
}

impl EncodeToSlice for VarInt16 {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        VarInt32::from(self.value as u32).encode_to_slice_unchecked(target)
    }
}

impl EncodeToWrite for VarInt16 {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, io::Error>
    where
        W: io::Write,
    {
        VarInt32::from(self.value as u32).encode_to_write(w)
    }
}

impl DecodeFromReadPrefix for VarInt16 {
    fn decode_from_read_prefix_with_first_byte<R>(first: u8, r: &mut R) -> Result<Self, io::Error>
    where
        R: io::Read,
    {
        let value: u32 = VarInt32::decode_from_read_prefix_with_first_byte(first, r)?.value;
        if value > (u16::MAX as u32) {
            Err(io::Error::new(InvalidData, InvalidEncodedData))
        } else {
            Ok(Self::from(value as u16))
        }
    }
}

impl Display for VarInt16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::io::Cursor;

    use crate::var_int::VarInt16;
    use crate::{DecodeFromReadPrefix, EncodeToSlice, EncodeToWrite, EncodedLen, Error};

    #[test]
    fn max_encoded_len() -> Result<(), Error> {
        assert_eq!(
            VarInt16::MAX_ENCODED_LEN,
            VarInt16::from(u16::MAX).encoded_len()?
        );
        Ok(())
    }

    const TEST_CASES: &[(u16, &[u8])] = &[
        (0x00, b"\x00"),       // 0 bits
        (0x01, b"\x01"),       // 1 bit
        (0x7F, b"\x7F"),       // highest one byte value
        (0x80, b"\x80\x01"),   // lowest two byte value
        (0x3FFF, b"\xFF\x7F"), // highest two byte value
        (u16::MAX, b"\xFF\xFF\x03"),
    ];

    #[test]
    fn encode() -> Result<(), io::Error> {
        for (value, expected) in TEST_CASES {
            let value: VarInt16 = VarInt16::from(value);

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
            let result: Result<VarInt16, io::Error> =
                VarInt16::decode_from_read_prefix(&mut cursor);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().value, *expected);

            let mut extra: Vec<u8> = input.to_vec();
            extra.push(0xFF);
            let mut cursor: Cursor<Vec<u8>> = Cursor::new(extra);
            let result: Result<VarInt16, io::Error> =
                VarInt16::decode_from_read_prefix(&mut cursor);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().value, *expected);
        }
        Ok(())
    }
}
