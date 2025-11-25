/// Decodes the `last_block` of 3 bytes into the `target`.
///
/// Decoding 3 bytes produces 18 bits, which is compressed to 2 bytes.
/// The last 2 bits of output are invalid and are discarded.
/// Returns the number of decoded bytes. (2)
///
/// # Safety
/// The `last_block` length must be exactly 3.
/// The `target` length must be exactly 2.
pub unsafe fn decode_block_last_3(
    table: &[u8; 256],
    last_block: &[u8],
    target: &mut [u8],
) -> usize {
    debug_assert_eq!(last_block.len(), 3);
    debug_assert_eq!(target.len(), 2);

    unsafe {
        let a: u32 = *table.get_unchecked(*last_block.get_unchecked(0) as usize) as u32;
        let b: u32 = *table.get_unchecked(*last_block.get_unchecked(1) as usize) as u32;
        let c: u32 = *table.get_unchecked(*last_block.get_unchecked(2) as usize) as u32;
        let bits: u32 = (a << 10) | (b << 4) | (c >> 2);
        *target.get_unchecked_mut(0) = (bits >> 8) as u8;
        *target.get_unchecked_mut(1) = bits as u8;
    }

    2
}

#[cfg(test)]
mod tests {
    use crate::base_64::decode::decode_block_last_3::decode_block_last_3;
    use crate::base_64::decode::decoding_table::DecodingTable;

    #[test]
    fn fn_decode_block() {
        let test_cases: &[(&str, &[u8])] = &[
            ("AAA", b"\x00\x00"),
            ("ZZZ", b"\x65\x96"),
            ("aaa", b"\x69\xA6"),
            ("zzz", b"\xCF\x3C"),
            ("000", b"\xD3\x4D"),
            ("///", b"\xFF\xFF"),
        ];

        let table: DecodingTable = DecodingTable::default();
        let table: &[u8; 256] = table.decoding_table();
        for (data, expected) in test_cases {
            let mut target: [u8; 2] = [0u8; 2];
            let result: usize = unsafe { decode_block_last_3(table, data.as_bytes(), &mut target) };
            assert_eq!(result, 2);
            assert_eq!(&target[..2], *expected, "data={}", *data);
        }
    }
}
