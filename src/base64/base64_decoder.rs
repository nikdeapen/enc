use crate::base64::decode_block::decode_block;
use crate::base64::decode_block_last::decode_block_last;
use crate::base64::decoded_len::decoded_len;
use crate::base64::decoding_table::DecodingTable;
use crate::base64::split_last_block::split_last_block;
use crate::Error::InsufficientTargetSpace;
use crate::{Decoder, Error};

/// Responsible for decoding base-64 encoded data.
#[derive(Clone, Debug)]
pub struct Base64Decoder {
    decoding_table: DecodingTable,
    padding: Option<u8>,
}

impl Base64Decoder {
    //! Construction

    /// Creates a new base-64 decoder.
    pub fn new(v63: u8, v64: u8, padding: Option<u8>) -> Self {
        Self {
            decoding_table: DecodingTable::get_decoding_table(v63, v64),
            padding,
        }
    }
}

impl Default for Base64Decoder {
    fn default() -> Self {
        Self::new(b'+', b'/', Some(b'='))
    }
}

impl Decoder for Base64Decoder {
    fn decoded_len(&self, data: &[u8]) -> Result<usize, Error> {
        Ok(decoded_len(self.padding, data))
    }

    fn decode_to_slice(&self, data: &[u8], target: &mut [u8]) -> Result<usize, Error> {
        let decoded_len: usize = self.decoded_len(data)?;
        if decoded_len > target.len() {
            Err(InsufficientTargetSpace)
        } else {
            let decoding_table: &[u8; 256] = self.decoding_table.decoding_table();
            let (full_blocks, last_block) = split_last_block(data);
            let mut d: usize = 0;
            let mut t: usize = 0;
            for _ in 0..(full_blocks.len() >> 2) {
                t += unsafe { decode_block(decoding_table, &full_blocks[d..], &mut target[t..]) };
                d += 4;
            }
            t += unsafe {
                decode_block_last(decoding_table, self.padding, last_block, &mut target[t..])
            };
            debug_assert_eq!(decoded_len, t);
            Ok(decoded_len)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base64::Base64Decoder;
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
        let decoder: Base64Decoder = Base64Decoder::default();
        for (data, expected) in test_cases {
            let result: Vec<u8> = decoder.decode_as_vec(data.as_bytes())?;
            assert_eq!(result, *expected, "data={}", *data);
        }
        Ok(())
    }
}
