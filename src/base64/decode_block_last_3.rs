/// Decodes the last block of 3 bytes without padding.
/// The last 4 bits of the 2nd decoded byte are ignored.
/// Returns the number of decoded bytes. (2)
#[inline(always)]
pub unsafe fn decode_block_last_3(
    decoding_table: &[u8; 256],
    data: &[u8],
    target: &mut [u8],
) -> usize {
    debug_assert_eq!(data.len(), 3);
    debug_assert_eq!(target.len(), 2);

    let a: u32 = *decoding_table.get_unchecked(data[0] as usize) as u32;
    let b: u32 = *decoding_table.get_unchecked(data[1] as usize) as u32;
    let c: u32 = *decoding_table.get_unchecked(data[2] as usize) as u32;
    let bits: u32 = (a << 10) | (b << 4) | (c >> 2);
    *target.get_unchecked_mut(0) = (bits >> 8) as u8;
    *target.get_unchecked_mut(1) = bits as u8;

    2
}

#[cfg(test)]
mod tests {
    use crate::base64::decode_block_last_3::decode_block_last_3;
    use crate::base64::decoding_table::DecodingTable;

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
        let decoding_table: DecodingTable = DecodingTable::default();
        let decoding_table: &[u8; 256] = decoding_table.decoding_table();
        for (data, expected) in test_cases {
            let mut target: [u8; 2] = [0u8; 2];
            let result: usize =
                unsafe { decode_block_last_3(decoding_table, data.as_bytes(), &mut target) };
            assert_eq!(result, 2);
            assert_eq!(&target[..2], *expected, "data={}", *data);
        }
    }
}
