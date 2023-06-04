use crate::percent::PercentDecoder;
use crate::Decoder;

#[test]
fn decoder() {
    let test_cases: &[(&str, &[u8])] = &[
        ("", b""),
        ("azAZ09", b"azAZ09"),
        ("%", b"%"),
        ("%0", b"%0"),
        ("%%0", b"%%0"),
        ("%0x", b"%0x"),
        ("%x0", b"%x0"),
        ("%01%23%45%67%89", b"\x01\x23\x45\x67\x89"),
        ("%AB%CD%EF", b"\xAB\xCD\xEF"),
        ("%ab%cd%ef", b"\xAB\xCD\xEF"),
    ];
    for (data, expected) in test_cases {
        let result: Vec<u8> = PercentDecoder::default()
            .decode_as_vec(data.as_bytes())
            .unwrap();
        assert_eq!(result, *expected);
    }
}
