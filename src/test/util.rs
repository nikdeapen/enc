use crate::hex::HexEncoder;

/// Converts the `data` to a human-readable hex string.
///
/// # Example
/// &[255, 255, 255] -> `FFFF:FF`
pub fn hex(data: &[u8]) -> String {
    let mut s: String = String::with_capacity(data.len() * 3);

    for (i, b) in data.iter().enumerate() {
        if i != 0 && i != data.len() && i % 2 == 0 {
            s.push(':');
        }
        let (a, b) = HexEncoder::UPPER.encode_chars(*b);
        s.push(a);
        s.push(b);
    }

    s
}

#[cfg(test)]
mod tests {
    use crate::test::hex;

    #[test]
    fn fn_hex() {
        let test_cases: &[(&[u8], &str)] = &[
            (&[], ""),
            (&[0x01], "01"),
            (&[0x01, 0x23], "0123"),
            (&[0x01, 0x23, 0x45], "0123:45"),
            (&[0x01, 0x23, 0x45, 0x67], "0123:4567"),
        ];

        for (input, expected) in test_cases {
            let result: String = hex(*input);
            assert_eq!(result.as_str(), *expected);
        }
    }
}
