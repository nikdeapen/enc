use crate::base_64::decode::decoded_len_last_block::decoded_length_last_block;

/// Gets the decoded length of the data.
///
/// The padding, if given, will be ignored. See `remove_padding_last_block`.
#[inline(always)]
pub fn decoded_len(data: &[u8], padding: Option<u8>) -> usize {
    let len: usize = data.len();
    if len == 0 {
        0
    } else {
        let rem: usize = len % 4;
        let last_chunk_index: usize = len - if rem == 0 { 4 } else { rem };
        let full: usize = (last_chunk_index / 4) * 3;
        let last: usize = unsafe { decoded_length_last_block(&data[last_chunk_index..], padding) };
        full + last
    }
}

#[cfg(test)]
mod tests {
    use crate::base_64::decode::decoded_len::decoded_len;

    #[test]
    fn fn_decoded_len() {
        let test_cases: &[(Option<u8>, &str, usize)] = &[
            (None, "", 0),
            (None, "A", 1),
            (None, "AA", 1),
            (None, "AAA", 2),
            (None, "AAAA", 3),
            (None, "AAAAA", 4),
            (None, "AAAAAA", 4),
            (None, "AAAAAAA", 5),
            (None, "AAAAAAAA", 6),
            (None, "AAAAAAAA====", 9),
            (Some(b'='), "AAAAAAAA====", 7),
            (None, "AAAAAAAAA===", 9),
            (Some(b'='), "AAAAAAAAA===", 7),
            (None, "AAAAAAAAAA==", 9),
            (Some(b'='), "AAAAAAAAAA==", 7),
            (None, "AAAAAAAAAAA=", 9),
            (Some(b'='), "AAAAAAAAAAA=", 8),
            (None, "AAAAAAAAAAAA", 9),
            (Some(b'='), "AAAAAAAAAAAA", 9),
        ];
        for (padding, data, expected) in test_cases {
            let result: usize = decoded_len(data.as_bytes(), *padding);
            assert_eq!(result, *expected, "pad={:?} data={}", *padding, *data);
        }
    }
}
