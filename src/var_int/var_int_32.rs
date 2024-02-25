use std::fmt::{Display, Formatter};
use std::io;

use crate::{EncodeToSlice, EncodeToWrite, EncodedLen};

/// A variable-length encoded `u32` value.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct VarInt32 {
    pub value: u32,
}

impl From<u32> for VarInt32 {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl From<&u32> for VarInt32 {
    fn from(value: &u32) -> Self {
        Self::from(*value)
    }
}

impl VarInt32 {
    //! Constants

    /// The maximum length of an encoded `u32` value.
    pub const MAX_ENCODED_LEN: usize = ((u32::BITS + 6) / 7) as usize;
}

impl EncodedLen for VarInt32 {
    fn encoded_len(&self) -> usize {
        let bits: u32 = u32::BITS - self.value.leading_zeros();
        if bits == 0 {
            1
        } else {
            ((bits + 6) / 7) as usize
        }
    }
}

impl EncodeToSlice for VarInt32 {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> usize {
        let mut t: usize = 0;
        let mut v: u32 = self.value;
        for _ in 0..(Self::MAX_ENCODED_LEN - 1) {
            let last_seven: u8 = (v & 0x7F) as u8;
            v >>= 7;
            if v == 0 {
                *target.get_unchecked_mut(t) = last_seven;
                return t + 1;
            } else {
                *target.get_unchecked_mut(t) = last_seven | 0x80;
                t += 1;
            }
        }
        *target.get_unchecked_mut(t) = v as u8;
        t + 1
    }
}

impl EncodeToWrite for VarInt32 {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, io::Error>
    where
        W: io::Write,
    {
        let mut buffer: [u8; Self::MAX_ENCODED_LEN] = [0u8; Self::MAX_ENCODED_LEN];
        let encoded_len: usize = unsafe { self.encode_to_slice_unchecked(&mut buffer) };
        w.write_all(&mut buffer[..encoded_len])?;
        Ok(encoded_len)
    }
}

impl Display for VarInt32 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::io::Cursor;

    use crate::var_int::VarInt32;
    use crate::{EncodeToSlice, EncodeToWrite, EncodedLen};

    #[test]
    fn max_encoded_len() {
        assert_eq!(
            VarInt32::MAX_ENCODED_LEN,
            VarInt32::from(u32::MAX).encoded_len()
        );
    }

    #[test]
    fn encode() -> Result<(), io::Error> {
        let test_cases: &[(u32, &[u8])] = &[
            (0x00, b"\x00"),                     // 0 bits
            (0x01, b"\x01"),                     // 1 bit
            (0x7F, b"\x7F"),                     // highest one byte value
            (0x80, b"\x80\x01"),                 // lowest two byte value
            (0x3FFF, b"\xFF\x7F"),               // highest two byte value
            (u32::MAX, b"\xFF\xFF\xFF\xFF\x0F"), // max
        ];
        for (value, expected) in test_cases {
            let value: VarInt32 = VarInt32::from(value);

            let encoded_len: usize = value.encoded_len();
            assert_eq!(encoded_len, expected.len(), "value={}", value);

            let encoded: Vec<u8> = value.encode_as_vec();
            assert_eq!(encoded, *expected, "value={}", value);

            let mut output: Cursor<Vec<u8>> = Cursor::default();
            value.encode_to_write(&mut output)?;
            assert_eq!(output.into_inner(), *expected);
        }
        Ok(())
    }
}
