use crate::Error::{InsufficientTargetSpace, InvalidEncodedData};
use crate::{Decoder, Error};

/// Responsible for decoding hex encoded data.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct HexDecoder {
    nothing: (),
}

impl Default for HexDecoder {
    fn default() -> Self {
        Self { nothing: () }
    }
}

impl HexDecoder {
    //! Decoding Table

    /// The invalid decoding byte.
    pub(crate) const INV: u8 = 0xFF;

    /// The decoding table.
    pub(crate) const TABLE: [u8; 256] = Self::create_decoding_table();

    /// Creates the decoding table.
    const fn create_decoding_table() -> [u8; 256] {
        let mut decoding_table: [u8; 256] = [Self::INV; 256];

        decoding_table['0' as usize] = 0x00;
        decoding_table['1' as usize] = 0x01;
        decoding_table['2' as usize] = 0x02;
        decoding_table['3' as usize] = 0x03;
        decoding_table['4' as usize] = 0x04;
        decoding_table['5' as usize] = 0x05;
        decoding_table['6' as usize] = 0x06;
        decoding_table['7' as usize] = 0x07;
        decoding_table['8' as usize] = 0x08;
        decoding_table['9' as usize] = 0x09;

        decoding_table['A' as usize] = 0x0A;
        decoding_table['B' as usize] = 0x0B;
        decoding_table['C' as usize] = 0x0C;
        decoding_table['D' as usize] = 0x0D;
        decoding_table['E' as usize] = 0x0E;
        decoding_table['F' as usize] = 0x0F;

        decoding_table['a' as usize] = 0x0A;
        decoding_table['b' as usize] = 0x0B;
        decoding_table['c' as usize] = 0x0C;
        decoding_table['d' as usize] = 0x0D;
        decoding_table['e' as usize] = 0x0E;
        decoding_table['f' as usize] = 0x0F;

        decoding_table
    }
}

impl HexDecoder {
    //! Decoding

    /// Decodes the high and low hex bytes as a byte.
    #[inline(always)]
    pub const fn decode_bytes(high: u8, low: u8) -> u8 {
        (Self::TABLE[high as usize] << 4) | Self::TABLE[low as usize]
    }

    /// Decodes the high and low hex chars as a byte.
    #[inline(always)]
    pub const fn decode_chars(high: char, low: char) -> u8 {
        Self::decode_bytes(high as u8, low as u8)
    }
}

impl Decoder for HexDecoder {
    fn decoded_len(&self, data: &[u8]) -> Result<usize, Error> {
        if data.len() % 2 == 0 {
            Ok(data.len() / 2)
        } else {
            Err(InvalidEncodedData)
        }
    }

    fn decode_slice(&self, data: &[u8], target: &mut [u8]) -> Result<usize, Error> {
        let decoded_len: usize = self.decoded_len(data)?;
        if target.len() < decoded_len {
            Err(InsufficientTargetSpace)
        } else {
            let target: &mut [u8] = &mut target[..decoded_len];
            for (d, t) in data.chunks_exact(2).zip(target.iter_mut()) {
                *t = Self::decode_bytes(d[0], d[1])
            }
            Ok(decoded_len)
        }
    }
}
