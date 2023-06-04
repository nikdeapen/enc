use crate::hex::HexDecoder;
use crate::Decoder;

#[test]
fn decode() {
    let test_cases: &[(&str, &[u8])] = &[
        ("", b""),
        ("0123456789", b"\x01\x23\x45\x67\x89"),
        ("1032547698", b"\x10\x32\x54\x76\x98"),
        ("ABCDEF", b"\xAB\xCD\xEF"),
        ("abcdef", b"\xAB\xCD\xEF"),
        ("BADCFE", b"\xBA\xDC\xFE"),
        ("badcfe", b"\xBA\xDC\xFE"),
    ];
    for (data, expected) in test_cases {
        let result: Vec<u8> = HexDecoder::default()
            .decode_as_vec(data.as_bytes())
            .unwrap();
        assert_eq!(result, *expected);
    }
}
