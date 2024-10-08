use crate::base_64::decode::decode_block::decode_block;
use crate::base_64::decode::decode_block_last_1::decode_block_last_1;
use crate::base_64::decode::decode_block_last_2::decode_block_last_2;
use crate::base_64::decode::decode_block_last_3::decode_block_last_3;
use crate::base_64::decode::remove_padding_last_block::remove_padding_last_block;

/// Decodes the last block of up to 4 bytes with padding.
///
/// Padding will be removed based on `remove_padding_last_block`.
/// Returns the number of decoded bytes. ([1, 3])
#[inline(always)]
pub unsafe fn decode_block_last(
    decoding_table: &[u8; 256],
    padding: Option<u8>,
    data: &[u8],
    target: &mut [u8],
) -> usize {
    debug_assert!(data.len() <= 4);

    let data: &[u8] = remove_padding_last_block(data, padding);
    match data.len() {
        0 => 0,
        1 => {
            debug_assert_eq!(target.len(), 1);
            decode_block_last_1(decoding_table, data, target)
        }
        2 => {
            debug_assert_eq!(target.len(), 1);
            decode_block_last_2(decoding_table, data, target)
        }
        3 => {
            debug_assert_eq!(target.len(), 2);
            decode_block_last_3(decoding_table, data, target)
        }
        4 => {
            debug_assert_eq!(target.len(), 3);
            decode_block(decoding_table, data, target)
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::base_64::decode::decode_block_last::decode_block_last;
    use crate::base_64::decode::decoding_table::DecodingTable;

    #[test]
    fn fn_decode_block_last() {
        let test_cases: &[(&str, &[u8])] = &[
            ("", b""),
            ("/", b"\xFC"),
            ("//", b"\xFF"),
            ("//=", b"\xFF"),
            ("//==", b"\xFF"),
            ("///", b"\xFF\xFF"),
            ("///=", b"\xFF\xFF"),
            ("////", b"\xFF\xFF\xFF"),
        ];
        let decoding_table: DecodingTable = DecodingTable::default();
        let decoding_table: &[u8; 256] = decoding_table.decoding_table();
        for (data, expected) in test_cases {
            let mut target: [u8; 3] = [0, 0, 0];
            let result: usize = unsafe {
                decode_block_last(
                    decoding_table,
                    Some(b'='),
                    data.as_bytes(),
                    &mut target[..expected.len()],
                )
            };
            assert_eq!(result, expected.len(), "data={}", *data);
            assert_eq!(&target[..expected.len()], *expected, "data={}", *data);
        }
    }
}
