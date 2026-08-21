use crate::base_64::as_array::as_array_mut;

/// Encodes the last `block` with two bytes of data.
///
/// Returns the number of encoded bytes: (3 or 4).
///
/// # Panics
/// Panics if the `target` length is not exactly 3, or exactly 4 with `padding`.
#[inline(always)]
pub fn encode_last_block_2(
    table: &[u8; 64],
    padding: Option<u8>,
    block: &[u8; 2],
    target: &mut [u8],
) -> usize {
    let bits: u32 = ((block[0] as u32) << 8) | (block[1] as u32);
    let a: u8 = table[(bits >> 10) as usize];
    let b: u8 = table[((bits >> 4) & 0x3F) as usize];
    let c: u8 = table[((bits << 2) & 0x3F) as usize];

    if let Some(padding) = padding {
        *as_array_mut::<4>(target) = [a, b, c, padding];
        4
    } else {
        *as_array_mut::<3>(target) = [a, b, c];
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
        let table: &[u8; 64] = table.table();
        for (input, no_pad, with_pad) in test_cases {
            let mut target: [u8; 4] = [0u8; 4];
            let result: usize = encode_last_block_2(table, None, input, &mut target[..3]);
            assert_eq!(result, 3);
            assert_eq!(&target[..3], no_pad.as_bytes(), "input={:?}", *input);

            let mut target: [u8; 4] = [0u8; 4];
            let result: usize = encode_last_block_2(table, Some(b'='), input, &mut target);
            assert_eq!(result, 4);
            assert_eq!(&target, with_pad.as_bytes(), "input={:?}", *input);
        }
    }
}
