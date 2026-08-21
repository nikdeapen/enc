use crate::base_64::as_array::{as_array, as_array_mut};
use crate::base_64::decode::{
    decode_block, decode_block_last_1, decode_block_last_2, decode_block_last_3,
    remove_padding_last_block,
};

/// Decodes the `last_block` of up to 4 bytes into the `target`.
///
/// Returns the number of decoded bytes. ([0, 3])
///
/// # Panics
/// Panics if the `last_block` is more than 4 bytes or if the `target` length is not exactly
/// `decoded_len_last_block(last_block, padding)`.
pub fn decode_block_last(
    table: &[u8; 256],
    padding: Option<u8>,
    last_block: &[u8],
    target: &mut [u8],
) -> usize {
    let last_block: &[u8] = remove_padding_last_block(last_block, padding);
    match last_block.len() {
        0 => {
            assert!(target.is_empty());
            0
        }
        1 => decode_block_last_1(table, as_array(last_block), as_array_mut(target)),
        2 => decode_block_last_2(table, as_array(last_block), as_array_mut(target)),
        3 => decode_block_last_3(table, as_array(last_block), as_array_mut(target)),
        4 => decode_block(table, as_array(last_block), as_array_mut(target)),
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

        let table: DecodingTable = DecodingTable::default();
        let table: &[u8; 256] = table.table();
        for (data, expected) in test_cases {
            let mut target: [u8; 3] = [0, 0, 0];
            let result: usize = decode_block_last(
                table,
                Some(b'='),
                data.as_bytes(),
                &mut target[..expected.len()],
            );
            assert_eq!(result, expected.len(), "data={}", *data);
            assert_eq!(&target[..expected.len()], *expected, "data={}", *data);
        }
    }
}
