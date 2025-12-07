use crate::base_64::decode::decode_block::decode_block;
use crate::base_64::decode::decode_block_last::decode_block_last;
use crate::base_64::decode::decoded_len::decoded_len;
use crate::base_64::decode::decoding_table::DecodingTable;
use crate::base_64::decode::split_last_block::split_last_block;
use crate::base_64::Base64Encoder;
use crate::Error::InsufficientTargetSpace;
use crate::{Decoder, Error};

/// Responsible for decoding base-64 encoded data.
///
/// # Validation
/// This decoder implementation does nothing to validate the encoded data. If invalid input data is
/// given, the output bytes are undefined. The decoded length calculation will still be accurate,
/// and decoding data will not cause a panic.
#[derive(Clone, Debug)]
pub struct Base64Decoder {
    table: DecodingTable,
    padding: Option<u8>,
}

impl Base64Decoder {
    //! Constants

    /// The block size for base-64 decoding.
    const BLOCK_SIZE: usize = 4;
}

impl Base64Decoder {
    //! Construction

    /// Creates a new base-64 decoder.
    ///
    /// Returns `None` if the decoding config is invalid.
    pub fn new(v63: u8, v64: u8, padding: Option<u8>) -> Option<Self> {
        if Base64Encoder::is_valid_config(v63, v64, padding) {
            Some(Self {
                table: DecodingTable::get_decoding_table(v63, v64),
                padding,
            })
        } else {
            None
        }
    }
}

impl Default for Base64Decoder {
    fn default() -> Self {
        Self::new(
            Base64Encoder::DEFAULT_V63,
            Base64Encoder::DEFAULT_V64,
            Base64Encoder::DEFAULT_PADDING,
        )
        .unwrap()
    }
}

impl Decoder for Base64Decoder {
    fn decoded_len(&self, data: &[u8]) -> Result<usize, Error> {
        Ok(decoded_len(data, self.padding))
    }

    fn decode_to_slice(&self, data: &[u8], target: &mut [u8]) -> Result<usize, Error> {
        let decoded_len: usize = self.decoded_len(data)?;
        if decoded_len > target.len() {
            Err(InsufficientTargetSpace)
        } else {
            let table: &[u8; 256] = self.table.decoding_table();
            let (full_blocks, last_block) = split_last_block(data);
            let mut d: usize = 0;
            let mut t: usize = 0;
            for _ in 0..(full_blocks.len() / Self::BLOCK_SIZE) {
                t += unsafe { decode_block(table, &full_blocks[d..], &mut target[t..]) };
                d += Self::BLOCK_SIZE;
            }
            t += unsafe { decode_block_last(table, self.padding, last_block, &mut target[t..]) };
            debug_assert_eq!(decoded_len, t);
            Ok(decoded_len)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base_64::Base64Decoder;
    use crate::{Decoder, Error};

    #[test]
    fn decode() -> Result<(), Error> {
        let test_cases: &[(&str, &[u8])] = &[
            ("", b""),
            ("AAAA", b"\x00\x00\x00"),
            ("////", b"\xFF\xFF\xFF"),
            ("/////", b"\xFF\xFF\xFF\xFC"),
            ("//////", b"\xFF\xFF\xFF\xFF"),
            ("//////=", b"\xFF\xFF\xFF\xFF"),
            ("//////==", b"\xFF\xFF\xFF\xFF"),
            ("///////", b"\xFF\xFF\xFF\xFF\xFF"),
            ("///////=", b"\xFF\xFF\xFF\xFF\xFF"),
            ("////////", b"\xFF\xFF\xFF\xFF\xFF\xFF"),
        ];

        // todo -- decoder testing
        let decoder: Base64Decoder = Base64Decoder::default();
        for (data, expected) in test_cases {
            let result: Vec<u8> = decoder.decode_as_vec(data.as_bytes())?;
            assert_eq!(result, *expected, "data={}", *data);
        }

        Ok(())
    }
}
