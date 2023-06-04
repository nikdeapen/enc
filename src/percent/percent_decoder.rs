use crate::hex::{HexDecoder, HexValidator};
use crate::Error::InsufficientTargetSpace;
use crate::{Decoder, Error};

/// Responsible for decoding percent encoded data.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct PercentDecoder {
    hex_validator: HexValidator,
}

impl Default for PercentDecoder {
    fn default() -> Self {
        Self {
            hex_validator: HexValidator::default(),
        }
    }
}

impl PercentDecoder {
    //! Decoding

    /// Checks if the prefix of the data is a properly encoded byte. This function assumes the
    /// first byte is a '%' and should only be used internally.
    #[inline(always)]
    const fn prefix_is_encoded(&self, data: &[u8]) -> bool {
        data.len() >= 3
            && self.hex_validator.is_valid_byte(data[1])
            && self.hex_validator.is_valid_byte(data[2])
    }
}

impl Decoder for PercentDecoder {
    fn decoded_len(&self, data: &[u8]) -> Result<usize, Error> {
        let encoded_count: usize = data
            .iter()
            .enumerate()
            .filter(|(_, b)| **b == b'%')
            .map(|(i, _)| &data[i..])
            .filter(|data| self.prefix_is_encoded(*data))
            .count();
        Ok(data.len() - (encoded_count * 2))
    }

    fn decode_slice(&self, data: &[u8], target: &mut [u8]) -> Result<usize, Error> {
        let mut d: usize = 0;
        let mut t: usize = 0;
        while d < data.len() {
            if t == target.len() {
                return Err(InsufficientTargetSpace);
            }
            if data[d] == b'%' && self.prefix_is_encoded(&data[d..]) {
                let b: u8 = HexDecoder::decode_bytes(data[d + 1], data[d + 2]);
                target[t] = b;
                d += 3;
            } else {
                target[t] = data[d];
                d += 1;
            }
            t += 1;
        }
        Ok(t)
    }
}
