/// Decodes the `last_block` of 3 bytes into the `target`.
///
/// Decoding 3 bytes produces 18 bits, which is compressed to 2 bytes.
/// The last 2 bits of output are invalid and are discarded.
/// Returns the number of decoded bytes. (2)
#[inline(always)]
pub fn decode_block_last_3(table: &[u8; 256], last_block: &[u8; 3], target: &mut [u8; 2]) -> usize {
    let a: u32 = table[last_block[0] as usize] as u32;
    let b: u32 = table[last_block[1] as usize] as u32;
    let c: u32 = table[last_block[2] as usize] as u32;
    let bits: u32 = (a << 10) | (b << 4) | (c >> 2);
    target[0] = (bits >> 8) as u8;
    target[1] = bits as u8;

    2
}

#[cfg(test)]
mod tests {
    use crate::base_64::decode::decode_block_last_3::decode_block_last_3;
    use crate::base_64::decode::decoding_table::DecodingTable;

    #[test]
    fn fn_decode_block() {
        let test_cases: &[(&[u8; 3], &[u8; 2])] = &[
            (b"AAA", b"\x00\x00"),
            (b"ZZZ", b"\x65\x96"),
            (b"aaa", b"\x69\xA6"),
            (b"zzz", b"\xCF\x3C"),
            (b"000", b"\xD3\x4D"),
            (b"///", b"\xFF\xFF"),
        ];

        let table: DecodingTable = DecodingTable::default();
        let table: &[u8; 256] = table.table();
        for (data, expected) in test_cases {
            let mut target: [u8; 2] = [0u8; 2];
            let result: usize = decode_block_last_3(table, data, &mut target);
            assert_eq!(result, 2);
            assert_eq!(&target, *expected, "data={:?}", *data);
        }
    }
}
