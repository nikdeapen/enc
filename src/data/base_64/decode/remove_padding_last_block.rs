/// Removes the `padding` from the `last_block`.
///
/// Returns the `last_block` without the `padding`.
pub unsafe fn remove_padding_last_block(last_block: &[u8], padding: Option<u8>) -> &[u8] {
    debug_assert!(last_block.len() <= 4);

    if last_block.len() <= 2 {
        last_block
    } else if let Some(padding) = padding {
        let c: u8 = unsafe { *last_block.get_unchecked(2) };
        if c == padding {
            &last_block[..2]
        } else if last_block.len() == 4 {
            let d: u8 = unsafe { *last_block.get_unchecked(3) };
            if d == padding {
                &last_block[..3]
            } else {
                last_block
            }
        } else {
            last_block
        }
    } else {
        last_block
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
            (Some(b'='), "AA=A", "AA"),
            (Some(b'='), "AAA=", "AAA"),
            (Some(b'='), "A==", "A="),
            (Some(b'='), "AA==", "AA"),
            (Some(b'='), "A===", "A="),
        ];

        for (padding, last_block, expected) in test_cases {
            let result: &[u8] =
                unsafe { remove_padding_last_block(last_block.as_bytes(), *padding) };
            assert_eq!(result, expected.as_bytes());
        }
    }
}
