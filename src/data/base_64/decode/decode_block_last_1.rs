/// Decodes the `last_block` of 1 byte into the `target`.
///
/// Decoding 1 byte produces 6 bits, which is expanded to 1 byte.
/// The last 2 bits of output are set to 0.
/// Returns the number of decoded bytes. (1)
#[inline(always)]
pub fn decode_block_last_1(table: &[u8; 256], last_block: &[u8; 1], target: &mut [u8; 1]) -> usize {
    let a: u32 = table[last_block[0] as usize] as u32;
    let bits: u32 = a << 2;
    target[0] = bits as u8;

    1
}

#[cfg(test)]
mod tests {
    use crate::base_64::decode::decode_block_last_1::decode_block_last_1;
    use crate::base_64::decode::decoding_table::DecodingTable;

    #[test]
    fn fn_decode_block() {
        let test_cases: &[(&[u8; 1], &[u8; 1])] = &[
            (b"A", b"\x00"),
            (b"Z", b"\x64"),
            (b"a", b"\x68"),
            (b"z", b"\xCC"),
            (b"/", b"\xFC"),
        ];

        let table: DecodingTable = DecodingTable::default();
        let table: &[u8; 256] = table.table();
        for (data, expected) in test_cases {
            let mut target: [u8; 1] = [0u8; 1];
            let result: usize = decode_block_last_1(table, data, &mut target);
            assert_eq!(result, 1);
            assert_eq!(&target, *expected, "data={:?}", *data);
        }
    }
}
