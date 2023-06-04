use crate::hex::HexValidator;
use crate::Validator;

#[test]
fn is_valid() {
    let test_cases: &[(&str, bool, bool)] = &[
        ("", true, true),
        ("0", false, false),
        ("00", true, true),
        ("000", false, false),
        ("0000", true, true),
        ("00000", false, false),
        ("0123456789", true, true),
        ("abcdef", true, false),
        ("ABCDEF", false, true),
        ("//", false, false),
        ("::", false, false),
        ("@@", false, false),
        ("GG", false, false),
        ("``", false, false),
        ("gg", false, false),
    ];
    for (data, expected_lower, expected_upper) in test_cases {
        let result: bool = HexValidator::CASELESS.is_valid(data.as_bytes());
        assert_eq!(result, *expected_lower || *expected_upper, "data={}", *data);

        let result: bool = HexValidator::LOWER_ONLY.is_valid(data.as_bytes());
        assert_eq!(result, *expected_lower, "data={}", *data);

        let result: bool = HexValidator::UPPER_ONLY.is_valid(data.as_bytes());
        assert_eq!(result, *expected_upper, "data={}", *data);
    }
}
