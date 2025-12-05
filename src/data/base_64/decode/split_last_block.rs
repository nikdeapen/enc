/// Splits the `data` into full blocks and the last block.
///
/// Returns `(full_blocks, last_block)`.
///
/// The last block will only be empty if `data` is empty, otherwise it will contain the last bytes
/// after all the full 4-byte blocks. If the length of the `data` is mod 4 it will return the
/// entire last 4 bytes so the last block can be stripped for padding.
pub fn split_last_block(data: &[u8]) -> (&[u8], &[u8]) {
    let len: usize = data.len();
    let last_block_index: usize = if len == 0 {
        0
    } else {
        let rem: usize = len % 4;
        if rem == 0 {
            len - 4
        } else {
            len & !0x03
        }
    };
    data.split_at(last_block_index)
}

#[cfg(test)]
mod tests {
    use crate::base_64::decode::split_last_block::split_last_block;

    #[test]
    fn fn_split_last_block() {
        let test_cases: &[(&str, &str, &str)] = &[
            ("", "", ""),
            ("A", "", "A"),
            ("AA", "", "AA"),
            ("AAA", "", "AAA"),
            ("AAAA", "", "AAAA"),
            ("AAAAA", "AAAA", "A"),
            ("AAAAAA", "AAAA", "AA"),
            ("AAAAAAA", "AAAA", "AAA"),
            ("AAAAAAAA", "AAAA", "AAAA"),
            ("AAAAAAAAA", "AAAAAAAA", "A"),
        ];

        for (data, expected_1, expected_2) in test_cases {
            let (result_1, result_2) = split_last_block(data.as_bytes());
            assert_eq!(result_1, expected_1.as_bytes(), "data={}", *data);
            assert_eq!(result_2, expected_2.as_bytes(), "data={}", *data)
        }
    }
}
