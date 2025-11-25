/// Decodes the `last_block` of 2 bytes into the `target`.
///
/// Decoding 2 bytes produces 12 bits, which is compressed to 1 byte.
/// The last 4 bits of output are invalid and are discarded.
/// Returns the number of decoded bytes. (1)
///
/// # Safety
/// The `last_block` length must be exactly 2.
/// The `target` length must be exactly 1.
pub unsafe fn decode_block_last_2(
    table: &[u8; 256],
    last_block: &[u8],
    target: &mut [u8],
) -> usize {
    debug_assert_eq!(last_block.len(), 2);
    debug_assert_eq!(target.len(), 1);

    unsafe {
        let a: u32 = *table.get_unchecked(*last_block.get_unchecked(0) as usize) as u32;
        let b: u32 = *table.get_unchecked(*last_block.get_unchecked(1) as usize) as u32;
        let bits: u32 = (a << 2) | (b >> 4);
        *target.get_unchecked_mut(0) = bits as u8;
    }

    1
}

#[cfg(test)]
mod tests {
    use crate::base_64::decode::decode_block_last_2::decode_block_last_2;
    use crate::base_64::decode::decoding_table::DecodingTable;

    #[test]
    fn fn_decode_block() {
        let test_cases: &[(&str, &[u8])] = &[
            ("AA", b"\x00"),
            ("ZZ", b"\x65"),
            ("aa", b"\x69"),
            ("zz", b"\xCF"),
            ("//", b"\xFF"),
        ];

        let table: DecodingTable = DecodingTable::default();
        let table: &[u8; 256] = table.decoding_table();
        for (data, expected) in test_cases {
            let mut target: [u8; 1] = [0u8; 1];
            let result: usize = unsafe { decode_block_last_2(table, data.as_bytes(), &mut target) };
            assert_eq!(result, 1);
            assert_eq!(&target[..1], *expected, "data={}", *data);
        }
    }
}
