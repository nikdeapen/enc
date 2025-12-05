use crate::base_64::decode::decode_block::decode_block;
use crate::base_64::decode::decode_block_last_1::decode_block_last_1;
use crate::base_64::decode::decode_block_last_2::decode_block_last_2;
use crate::base_64::decode::decode_block_last_3::decode_block_last_3;
use crate::base_64::decode::remove_padding_last_block::remove_padding_last_block;

/// Decodes the `last_block` of up to 4 bytes into the `target`.
///
/// Returns the number of decoded bytes. ([0, 3])
///
/// # Safety
/// The `last_block` length must be at most 4.
pub unsafe fn decode_block_last(
    table: &[u8; 256],
    padding: Option<u8>,
    last_block: &[u8],
    target: &mut [u8],
) -> usize {
    debug_assert!(last_block.len() <= 4);

    unsafe {
        let last_block: &[u8] = remove_padding_last_block(last_block, padding);
        match last_block.len() {
            0 => {
                debug_assert_eq!(target.len(), 0);
                0
            }
            1 => {
                debug_assert_eq!(target.len(), 1);
                decode_block_last_1(table, last_block, target)
            }
            2 => {
                debug_assert_eq!(target.len(), 1);
                decode_block_last_2(table, last_block, target)
            }
            3 => {
                debug_assert_eq!(target.len(), 2);
                decode_block_last_3(table, last_block, target)
            }
            4 => {
                debug_assert_eq!(target.len(), 3);
                decode_block(table, last_block, target)
            }
            _ => unreachable!(),
        }
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

        let table: DecodingTable = DecodingTable::default();
        let table: &[u8; 256] = table.decoding_table();
        for (data, expected) in test_cases {
            let mut target: [u8; 3] = [0, 0, 0];
            let result: usize = unsafe {
                decode_block_last(
                    table,
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
