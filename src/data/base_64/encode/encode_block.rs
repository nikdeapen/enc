/// Encodes the full `block` of data.
///
/// This will encode 3 `block` bytes into 4 `target` bytes.
///
/// # Panics
/// Panics if `block` is less than 3 bytes or `target` is less than 4 bytes.
#[inline(always)]
pub fn encode_block(table: &[u8; 64], block: &[u8], target: &mut [u8]) {
    debug_assert!(block.len() >= 3);
    debug_assert!(target.len() >= 4);

    let a: u32 = block[0] as u32;
    let b: u32 = block[1] as u32;
    let c: u32 = block[2] as u32;
    let bits: u32 = (a << 16) | (b << 8) | c;

    let ai: usize = (bits >> 18) as usize;
    let bi: usize = ((bits >> 12) & 0x3F) as usize;
    let ci: usize = ((bits >> 6) & 0x3F) as usize;
    let di: usize = (bits & 0x3F) as usize;

    target[0] = table[ai];
    target[1] = table[bi];
    target[2] = table[ci];
    target[3] = table[di];
}

#[cfg(test)]
mod tests {
    use crate::base_64::encode::encode_block::encode_block;
    use crate::base_64::encode::encoding_table::EncodingTable;

    #[test]
    fn fn_encode_block() {
        let test_cases: &[(&[u8], &str)] = &[
            (b"\x00\x10\x83", "ABCD"),
            (b"\x10\x51\x87", "EFGH"),
            (b"\x20\x92\x8B", "IJKL"),
            (b"\x30\xD3\x8F", "MNOP"),
            (b"\x41\x14\x93", "QRST"),
            (b"\x51\x55\x97", "UVWX"),
            (b"\x61\x96\x9B", "YZab"),
            (b"\x71\xD7\x9F", "cdef"),
            (b"\x82\x18\xA3", "ghij"),
            (b"\x92\x59\xA7", "klmn"),
            (b"\xA2\x9A\xAB", "opqr"),
            (b"\xB2\xDB\xAF", "stuv"),
            (b"\xC3\x1C\xB3", "wxyz"),
            (b"\xD3\x5D\xB7", "0123"),
            (b"\xE3\x9E\xBB", "4567"),
            (b"\xF3\xDF\xBF", "89+/"),
        ];

        let table: EncodingTable = EncodingTable::default();
        let table: &[u8; 64] = table.encoding_table();
        for (data, expected) in test_cases {
            let mut target: [u8; 4] = [0u8; 4];
            encode_block(table, data, &mut target);
            assert_eq!(&target, expected.as_bytes(), "data={:?}", *data);
        }
    }
}
