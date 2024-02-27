/// Removes the padding from the last block of data. Returns the data without the padding.
pub unsafe fn remove_padding_last_block(data: &[u8], padding: Option<u8>) -> &[u8] {
    debug_assert!(data.len() <= 4);

    if data.len() < 3 {
        data
    } else {
        if let Some(padding) = padding {
            let c: u8 = *data.get_unchecked(2);
            if c == padding {
                &data[..2]
            } else {
                let d: u8 = *data.get_unchecked(3);
                if d == padding {
                    &data[..3]
                } else {
                    data
                }
            }
        } else {
            data
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base64::remove_padding_last_block::remove_padding_last_block;

    #[test]
    fn fn_remove_padding_last_block() {
        let test_cases: &[(Option<u8>, &str, &str)] = &[
            (None, "AA==", "AA=="),
            (Some(b'='), "AA==", "AA"),
            (Some(b'='), "AA=A", "AA"),
            (Some(b'='), "AA=", "AA"),
            (Some(b'='), "AA==", "AA"),
            (Some(b'='), "AAA=", "AAA"),
            (Some(b'='), "AAAA", "AAAA"),
        ];
        for (padding, data, expected) in test_cases {
            let result: &[u8] = unsafe { remove_padding_last_block(data.as_bytes(), *padding) };
            assert_eq!(result, expected.as_bytes());
        }
    }
}
