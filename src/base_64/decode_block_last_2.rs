/// Decodes the last block of 2 bytes without padding.
/// The last 4 bits of the 2nd decoded byte are ignored.
/// Returns the number of decoded bytes. (1)
#[inline(always)]
pub unsafe fn decode_block_last_2(
    decoding_table: &[u8; 256],
    data: &[u8],
    target: &mut [u8],
) -> usize {
    debug_assert_eq!(data.len(), 2);
    debug_assert_eq!(target.len(), 1);

    let a: u32 = *decoding_table.get_unchecked(data[0] as usize) as u32;
    let b: u32 = *decoding_table.get_unchecked(data[1] as usize) as u32;
    let bits: u32 = (a << 2) | (b >> 4);
    *target.get_unchecked_mut(0) = bits as u8;

    1
}

#[cfg(test)]
mod tests {
    use crate::base_64::decode_block_last_2::decode_block_last_2;
    use crate::base_64::decoding_table::DecodingTable;

    #[test]
    fn fn_decode_block() {
        let test_cases: &[(&str, &[u8])] = &[
            ("AA", b"\x00"),
            ("ZZ", b"\x65"),
            ("aa", b"\x69"),
            ("zz", b"\xCF"),
            ("//", b"\xFF"),
        ];
        let decoding_table: DecodingTable = DecodingTable::default();
        let decoding_table: &[u8; 256] = decoding_table.decoding_table();
        for (data, expected) in test_cases {
            let mut target: [u8; 1] = [0u8; 1];
            let result: usize =
                unsafe { decode_block_last_2(decoding_table, data.as_bytes(), &mut target) };
            assert_eq!(result, 1);
            assert_eq!(&target[..1], *expected, "data={}", *data);
        }
    }
}
