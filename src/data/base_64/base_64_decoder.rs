use crate::Error::InsufficientTargetSpace;
use crate::base_64::Base64Encoder;
use crate::base_64::constants::{DECODED_BLOCK_SIZE, ENCODED_BLOCK_SIZE};
use crate::base_64::decode;
use crate::base_64::decode::DecodingTable;
use crate::{Decoder, Error};
use std::fmt::{Debug, Formatter};

/// Responsible for decoding base-64 encoded data.
///
/// # Validation
/// This decoder implementation does nothing to validate the encoded data. If invalid input data is
/// given, the output bytes are undefined. The decoded length calculation will still be accurate,
/// and decoding data will not cause a panic.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Base64Decoder {
    table: DecodingTable,
    padding: Option<u8>,
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

impl Base64Decoder {
    //! Special Decoders

    /// Gets the URL-safe decoder.
    pub fn url_safe_decoder() -> Self {
        Self::new(
            Base64Encoder::URL_SAFE_V63,
            Base64Encoder::URL_SAFE_V64,
            Base64Encoder::URL_SAFE_PADDING,
        )
        .unwrap()
    }
}

impl Default for Base64Decoder {
    fn default() -> Self {
        Self {
            table: DecodingTable::default(),
            padding: Base64Encoder::DEFAULT_PADDING,
        }
    }
}

impl Decoder for Base64Decoder {
    fn decoded_len(&self, data: &[u8]) -> Result<usize, Error> {
        Ok(decode::decoded_len(data, self.padding))
    }

    fn decode_to_slice(&self, data: &[u8], target: &mut [u8]) -> Result<usize, Error> {
        let decoded_len: usize = self.decoded_len(data)?;
        if decoded_len > target.len() {
            Err(InsufficientTargetSpace)
        } else {
            let target: &mut [u8] = &mut target[..decoded_len];
            let table: &[u8; 256] = self.table.table();

            let (full_blocks, last_block) = decode::split_last_block(data);
            let blocks: &[[u8; ENCODED_BLOCK_SIZE]] =
                full_blocks.as_chunks::<ENCODED_BLOCK_SIZE>().0;
            let (block_targets, last_target) =
                target.split_at_mut(blocks.len() * DECODED_BLOCK_SIZE);
            for (block, block_target) in blocks.iter().zip(
                block_targets
                    .as_chunks_mut::<DECODED_BLOCK_SIZE>()
                    .0
                    .iter_mut(),
            ) {
                decode::decode_block(table, block, block_target);
            }

            let mut t: usize = blocks.len() * DECODED_BLOCK_SIZE;
            t += decode::decode_block_last(table, self.padding, last_block, last_target);
            debug_assert_eq!(decoded_len, t);
            Ok(decoded_len)
        }
    }
}

impl Debug for Base64Decoder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (v63, v64) = self.table.values();
        f.debug_struct("Base64Decoder")
            .field("v63", &(v63 as char))
            .field("v64", &(v64 as char))
            .field("padding", &self.padding.map(|p| p as char))
            .finish()
    }
}

#[cfg(test)]
#[cfg(feature = "dev")]
mod tests {
    use crate::base_64::{Base64Decoder, Base64Encoder};
    use crate::test::{test_decoder, test_round_trip};

    #[test]
    fn round_trip() {
        let configs: &[(u8, u8, Option<u8>)] = &[
            (b'+', b'/', Some(b'=')),
            (b'+', b'/', None),
            (b'-', b'_', None),
            (b'-', b'_', Some(b'=')),
            (b'!', b'@', Some(b'#')),
        ];
        for (v63, v64, padding) in configs {
            let encoder: Base64Encoder = Base64Encoder::new(*v63, *v64, *padding).unwrap();
            let decoder: Base64Decoder = Base64Decoder::new(*v63, *v64, *padding).unwrap();
            test_round_trip(&encoder, &decoder, 200);
        }
    }

    #[test]
    fn url_safe_decoder() {
        let expected: Base64Decoder = Base64Decoder::new(
            Base64Encoder::URL_SAFE_V63,
            Base64Encoder::URL_SAFE_V64,
            Base64Encoder::URL_SAFE_PADDING,
        )
        .unwrap();
        assert_eq!(Base64Decoder::url_safe_decoder(), expected);
    }

    #[test]
    fn decode() {
        let test_cases: &[(&str, &[u8])] = &[
            ("", b""),
            // full blocks
            ("AAAA", b"\x00\x00\x00"),
            ("89+/", b"\xF3\xDF\xBF"),
            ("////", b"\xFF\xFF\xFF"),
            ("AAAAAAAA", b"\x00\x00\x00\x00\x00\x00"),
            ("////////", b"\xFF\xFF\xFF\xFF\xFF\xFF"),
            // last blocks with padding
            ("AA==", b"\x00"),
            ("Ag==", b"\x02"),
            ("/w==", b"\xFF"),
            ("AAA=", b"\x00\x00"),
            ("AAE=", b"\x00\x01"),
            ("//8=", b"\xFF\xFF"),
            // multi-block with trailing partial
            ("/////", b"\xFF\xFF\xFF\xFC"),
            ("//////", b"\xFF\xFF\xFF\xFF"),
            ("//////=", b"\xFF\xFF\xFF\xFF"),
            ("//////==", b"\xFF\xFF\xFF\xFF"),
            ("///////", b"\xFF\xFF\xFF\xFF\xFF"),
            ("///////=", b"\xFF\xFF\xFF\xFF\xFF"),
        ];
        let decoder: Base64Decoder = Base64Decoder::default();
        test_decoder(&decoder, test_cases);
    }
}
