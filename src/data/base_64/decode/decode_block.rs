/// Decodes a full `block` of 4 bytes without padding into the `target`.
///
/// Decoding 4 bytes produces 24 bits, which is exactly 3 bytes.
/// Returns the number of decoded bytes. (3)
///
/// # Safety
/// The `block` length must be at least 4.
/// The `target` length must be at least 3.
pub unsafe fn decode_block(table: &[u8; 256], block: &[u8], target: &mut [u8]) -> usize {
    debug_assert!(block.len() >= 4);
    debug_assert!(target.len() >= 3);

    unsafe {
        let a: u32 = *table.get_unchecked(*block.get_unchecked(0) as usize) as u32;
        let b: u32 = *table.get_unchecked(*block.get_unchecked(1) as usize) as u32;
        let c: u32 = *table.get_unchecked(*block.get_unchecked(2) as usize) as u32;
        let d: u32 = *table.get_unchecked(*block.get_unchecked(3) as usize) as u32;

        let bits: u32 = (a << 18) | (b << 12) | (c << 6) | d;

        *target.get_unchecked_mut(0) = (bits >> 16) as u8;
        *target.get_unchecked_mut(1) = (bits >> 8) as u8;
        *target.get_unchecked_mut(2) = bits as u8;
    }

    3
}

#[cfg(test)]
mod tests {
    use crate::base_64::decode::decode_block::decode_block;
    use crate::base_64::decode::decoding_table::DecodingTable;

    #[test]
    fn fn_decode_block() {
        let test_cases: &[(&str, &[u8])] = &[
            ("ABCD", b"\x00\x10\x83"),
            ("EFGH", b"\x10\x51\x87"),
            ("IJKL", b"\x20\x92\x8B"),
            ("MNOP", b"\x30\xD3\x8F"),
            ("QRST", b"\x41\x14\x93"),
            ("UVWX", b"\x51\x55\x97"),
            ("YZab", b"\x61\x96\x9B"),
            ("cdef", b"\x71\xD7\x9F"),
            ("ghij", b"\x82\x18\xA3"),
            ("klmn", b"\x92\x59\xA7"),
            ("opqr", b"\xA2\x9A\xAB"),
            ("stuv", b"\xB2\xDB\xAF"),
            ("wxyz", b"\xC3\x1C\xB3"),
            ("0123", b"\xD3\x5D\xB7"),
            ("4567", b"\xE3\x9E\xBB"),
            ("89+/", b"\xF3\xDF\xBF"),
        ];

        let table: DecodingTable = DecodingTable::default();
        let table: &[u8; 256] = table.decoding_table();
        for (data, expected) in test_cases {
            let mut target: [u8; 4] = [0u8; 4];
            let result: usize = unsafe { decode_block(table, data.as_bytes(), &mut target) };
            assert_eq!(result, 3);
            assert_eq!(target[3], 0x00);
            assert_eq!(&target[..3], *expected, "data={}", *data);
        }
    }
}
