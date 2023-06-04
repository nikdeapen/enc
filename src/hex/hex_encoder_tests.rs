use crate::hex::HexEncoder;
use crate::StringEncoder;

#[test]
fn encode() {
    let test_cases: &[(&[u8], &str)] = &[
        (b"", ""),
        (b"\x01\x23\x45\x67\x89", "0123456789"),
        (b"\x10\x32\x54\x76\x98", "1032547698"),
        (b"\xAB\xCD\xEF", "abcdef"),
        (b"\xBA\xDC\xFE", "badcfe"),
    ];
    for (data, expected) in test_cases {
        let result: String = HexEncoder::LOWER.encode_as_string(*data).unwrap();
        assert_eq!(result, *expected);

        let result: String = HexEncoder::UPPER.encode_as_string(*data).unwrap();
        assert_eq!(result, expected.to_ascii_uppercase());
    }
}
