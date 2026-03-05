/// Encodes the last `block` with one byte of data.
///
/// Returns the number of encoded bytes: (2 or 4).
///
/// # Panics
/// Panics if `block` is not 1 byte or `target` is less than 2 bytes.
#[inline(always)]
pub fn encode_last_block_1(
    table: &[u8; 64],
    padding: Option<u8>,
    block: &[u8],
    target: &mut [u8],
) -> usize {
    debug_assert_eq!(block.len(), 1);
    debug_assert!(target.len() >= 2);

    let bits: u32 = block[0] as u32;
    let ai: usize = (bits >> 2) as usize;
    let bi: usize = ((bits << 4) & 0x3F) as usize;

    target[0] = table[ai];
    target[1] = table[bi];

    if let Some(padding) = padding {
        debug_assert_eq!(target.len(), 4);
        target[2] = padding;
        target[3] = padding;
        4
    } else {
        debug_assert_eq!(target.len(), 2);
        2
    }
}

#[cfg(test)]
mod tests {
    use crate::base_64::encode::encode_last_block_1::encode_last_block_1;
    use crate::base_64::encode::encoding_table::EncodingTable;

    #[test]
    fn fn_encode_last_block_1() {
        let test_cases: &[(u8, &str, &str)] = &[
            (b'\x00', "AA", "AA=="),
            (b'\xFF', "/w", "/w=="),
            (b'\xFC', "/A", "/A=="),
        ];

        let table: EncodingTable = EncodingTable::default();
        let table: &[u8; 64] = table.encoding_table();
        for (input, no_pad, with_pad) in test_cases {
            let mut target: [u8; 4] = [0u8; 4];
            let result: usize = encode_last_block_1(table, None, &[*input], &mut target[..2]);
            assert_eq!(result, 2);
            assert_eq!(&target[..2], no_pad.as_bytes(), "input={:#04X}", *input);

            let mut target: [u8; 4] = [0u8; 4];
            let result: usize = encode_last_block_1(table, Some(b'='), &[*input], &mut target);
            assert_eq!(result, 4);
            assert_eq!(&target, with_pad.as_bytes(), "input={:#04X}", *input);
        }
    }
}
