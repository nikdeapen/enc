/// Decodes a full `block` of 4 bytes without padding into the `target`.
///
/// Decoding 4 bytes produces 24 bits, which is exactly 3 bytes.
/// Returns the number of decoded bytes. (3)
#[inline(always)]
pub fn decode_block(table: &[u8; 256], block: &[u8; 4], target: &mut [u8; 3]) -> usize {
    let a: u32 = table[block[0] as usize] as u32;
    let b: u32 = table[block[1] as usize] as u32;
    let c: u32 = table[block[2] as usize] as u32;
    let d: u32 = table[block[3] as usize] as u32;

    let bits: u32 = (a << 18) | (b << 12) | (c << 6) | d;

    target[0] = (bits >> 16) as u8;
    target[1] = (bits >> 8) as u8;
    target[2] = bits as u8;

    3
}

#[cfg(test)]
mod tests {
    use crate::base_64::decode::decode_block::decode_block;
    use crate::base_64::decode::decoding_table::DecodingTable;

    #[test]
    fn fn_decode_block() {
        let test_cases: &[(&[u8; 4], &[u8])] = &[
            (b"ABCD", b"\x00\x10\x83"),
            (b"EFGH", b"\x10\x51\x87"),
            (b"IJKL", b"\x20\x92\x8B"),
            (b"MNOP", b"\x30\xD3\x8F"),
            (b"QRST", b"\x41\x14\x93"),
            (b"UVWX", b"\x51\x55\x97"),
            (b"YZab", b"\x61\x96\x9B"),
            (b"cdef", b"\x71\xD7\x9F"),
            (b"ghij", b"\x82\x18\xA3"),
            (b"klmn", b"\x92\x59\xA7"),
            (b"opqr", b"\xA2\x9A\xAB"),
            (b"stuv", b"\xB2\xDB\xAF"),
            (b"wxyz", b"\xC3\x1C\xB3"),
            (b"0123", b"\xD3\x5D\xB7"),
            (b"4567", b"\xE3\x9E\xBB"),
            (b"89+/", b"\xF3\xDF\xBF"),
        ];

        let table: DecodingTable = DecodingTable::default();
        let table: &[u8; 256] = table.table();
        for (data, expected) in test_cases {
            let mut target: [u8; 3] = [0u8; 3];
            let result: usize = decode_block(table, data, &mut target);
            assert_eq!(result, 3);
            assert_eq!(&target, *expected, "data={:?}", *data);
        }
    }
}
