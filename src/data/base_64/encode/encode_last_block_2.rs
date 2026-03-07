/// Encodes the last `block` with two bytes of data.
///
/// Returns the number of encoded bytes: (3 or 4).
///
/// # Panics
/// Panics if `block` is not 2 bytes or `target` is less than 3 bytes.
#[inline(always)]
pub fn encode_last_block_2(
    table: &[u8; 64],
    padding: Option<u8>,
    block: &[u8],
    target: &mut [u8],
) -> usize {
    debug_assert_eq!(block.len(), 2);
    debug_assert!(target.len() >= 3);

    let a: u32 = block[0] as u32;
    let b: u32 = block[1] as u32;
    let bits: u32 = (a << 8) | b;
    let ai: usize = (bits >> 10) as usize;
    let bi: usize = ((bits >> 4) & 0x3F) as usize;
    let ci: usize = ((bits << 2) & 0x3F) as usize;

    target[0] = table[ai];
    target[1] = table[bi];
    target[2] = table[ci];

    if let Some(padding) = padding {
        debug_assert_eq!(target.len(), 4);
        target[3] = padding;
        4
    } else {
        debug_assert_eq!(target.len(), 3);
        3
    }
}

#[cfg(test)]
mod tests {
    use crate::base_64::encode::encode_last_block_2::encode_last_block_2;
    use crate::base_64::encode::encoding_table::EncodingTable;

    #[test]
    fn fn_encode_last_block_2() {
        let test_cases: &[(&[u8; 2], &str, &str)] =
            &[(b"\x00\x00", "AAA", "AAA="), (b"\xFF\xFF", "//8", "//8=")];

        let table: EncodingTable = EncodingTable::default();
        let table: &[u8; 64] = table.encoding_table();
        for (input, no_pad, with_pad) in test_cases {
            let mut target: [u8; 4] = [0u8; 4];
            let result: usize = encode_last_block_2(table, None, *input, &mut target[..3]);
            assert_eq!(result, 3);
            assert_eq!(&target[..3], no_pad.as_bytes(), "input={:?}", *input);

            let mut target: [u8; 4] = [0u8; 4];
            let result: usize = encode_last_block_2(table, Some(b'='), *input, &mut target);
            assert_eq!(result, 4);
            assert_eq!(&target, with_pad.as_bytes(), "input={:?}", *input);
        }
    }
}
