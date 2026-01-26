use crate::base_64::decode::remove_padding_last_block::remove_padding_last_block;

/// Gets the length of the decoded `last_block`.
///
/// # Safety
/// The `last_block` length must be at most 4.
pub unsafe fn decoded_length_last_block(last_block: &[u8], padding: Option<u8>) -> usize {
    debug_assert!(last_block.len() <= 4);

    let data: &[u8] = unsafe { remove_padding_last_block(last_block, padding) };
    match data.len() {
        0 => 0, // the data is empty
        1 => 1, // this is invalid, we assume two more 0 bits
        2 => 1, // this may be invalid, we discard the last 4 bits
        3 => 2, // this may be invalid, we discard the last 2 bits
        4 => 3, // a full block of data
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::base_64::decode::decoded_len_last_block::decoded_length_last_block;

    #[test]
    fn fn_decoded_len_last_block() {
        let test_cases: &[(Option<u8>, &str, usize)] = &[
            (None, "", 0),
            (None, "A", 1),
            (None, "AA", 1),
            (None, "AAA", 2),
            (None, "AAAA", 3),
            (None, "====", 3),
            (Some(b'='), "====", 1),
            (None, "A===", 3),
            (Some(b'='), "A===", 1),
            (None, "AA==", 3),
            (Some(b'='), "AA==", 1),
            (None, "AAA=", 3),
            (Some(b'='), "AAA=", 2),
            (None, "AAAA", 3),
            (Some(b'='), "AAAA", 3),
        ];

        for (padding, data, expected) in test_cases {
            let result: usize = unsafe { decoded_length_last_block(data.as_bytes(), *padding) };
            assert_eq!(result, *expected, "pad={:?} data={}", *padding, *data);
        }
    }
}
