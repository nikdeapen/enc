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

    /// Converts a signed i16 to an unsigned u16 using zigzag encoding.
    ///
    /// Zigzag encoding maps signed integers to unsigned integers so that numbers
    /// with a small absolute value have a small encoded value too.
    ///
    /// # Examples
    ///
    /// ```
    /// use enc::var_int::VarInt16;
    /// assert_eq!(VarInt16::zigzag_encode(0), 0);
    /// assert_eq!(VarInt16::zigzag_encode(-1), 1);
    /// assert_eq!(VarInt16::zigzag_encode(1), 2);
    /// assert_eq!(VarInt16::zigzag_encode(-2), 3);
    /// ```
    pub fn zigzag_encode(value: i16) -> u16 {
        ((value << 1) ^ (value >> 15)) as u16
    }

    /// Converts an unsigned u16 to a signed i16 using zigzag decoding.
    ///
    /// # Examples
    ///
    /// ```
    /// use enc::var_int::VarInt16;
    /// assert_eq!(VarInt16::zigzag_decode(0), 0);
    /// assert_eq!(VarInt16::zigzag_decode(1), -1);
    /// assert_eq!(VarInt16::zigzag_decode(2), 1);
    /// assert_eq!(VarInt16::zigzag_decode(3), -2);
    /// ```
    pub fn zigzag_decode(value: u16) -> i16 {
        ((value >> 1) as i16) ^ (-((value & 1) as i16))
    }

    /// Creates a VarInt16 from a signed i16 value using zigzag encoding.
    pub fn from_i16(value: i16) -> Self {
        Self {
            value: Self::zigzag_encode(value),
        }
    }

    /// Converts this VarInt16 to a signed i16 value using zigzag decoding.
    pub fn to_i16(&self) -> i16 {
        Self::zigzag_decode(self.value)
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
            assert_eq!(result?.value, *expected);

            let mut extra: Vec<u8> = input.to_vec();
            extra.push(0xFF);
            let mut cursor: Cursor<Vec<u8>> = Cursor::new(extra);
            let result: Result<VarInt16, io::Error> =
                VarInt16::decode_from_read_prefix(&mut cursor);
            assert!(result.is_ok());
            assert_eq!(result?.value, *expected);
        }
        Ok(())
    }

    #[test]
    fn zigzag_encoding() {
        // Test cases: (signed, unsigned)
        let test_cases = [
            (0i16, 0u16),
            (1i16, 2u16),
            (-1i16, 1u16),
            (2i16, 4u16),
            (-2i16, 3u16),
            (i16::MAX, (i16::MAX as u16) * 2),
            (i16::MIN, u16::MAX),
        ];

        for (signed, unsigned) in test_cases {
            // Test encoding
            assert_eq!(VarInt16::zigzag_encode(signed), unsigned);

            // Test decoding
            assert_eq!(VarInt16::zigzag_decode(unsigned), signed);

            // Test convenience methods
            let var_int = VarInt16::from_i16(signed);
            assert_eq!(var_int.value, unsigned);
            assert_eq!(var_int.to_i16(), signed);
        }
    }
}
