/// Removes the padding from the last block of data.
///
/// Returns the data without the padding.
#[inline(always)]
pub unsafe fn remove_padding_last_block(data: &[u8], padding: Option<u8>) -> &[u8] {
    debug_assert!(data.len() <= 4);

    if data.len() <= 2 {
        data
    } else {
        if let Some(padding) = padding {
            let c: u8 = *data.get_unchecked(2);
            if c == padding {
                &data[..2]
            } else if data.len() == 4 {
                let d: u8 = *data.get_unchecked(3);
                if d == padding {
                    &data[..3]
                } else {
                    data
                }
            } else {
                data
            }
        } else {
            data
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base_64::decode::remove_padding_last_block::remove_padding_last_block;

    #[test]
    fn fn_remove_padding_last_block() {
        let test_cases: &[(Option<u8>, &str, &str)] = &[
            (None, "AA==", "AA=="),
            (Some(b'='), "", ""),
            (Some(b'='), "A", "A"),
            (Some(b'='), "AA", "AA"),
            (Some(b'='), "AAA", "AAA"),
            (Some(b'='), "AAAA", "AAAA"),
            (Some(b'='), "A=", "A="),
            (Some(b'='), "AA=", "AA"),
            (Some(b'='), "AAA=", "AAA"),
            (Some(b'='), "A==", "A="),
            (Some(b'='), "AA==", "AA"),
            (Some(b'='), "A===", "A="),
        ];
        for (padding, data, expected) in test_cases {
            let result: &[u8] = unsafe { remove_padding_last_block(data.as_bytes(), *padding) };
            assert_eq!(result, expected.as_bytes());
        }
    }
}
