use crate::percent::PercentValidator;
use crate::Validator;

#[test]
fn is_valid() {
    let test_cases: &[(&str, bool)] = &[
        ("", true),
        ("azAZ09", true),
        ("+-.", true),
        ("!", false),
        ("@", false),
        ("#", false),
        (" ", false),
        ("%", false),
        ("%0", false),
        ("%0x", false),
        ("%x0", false),
        ("%00%09", true),
        ("%af%AF", true),
    ];
    let validator: PercentValidator = PercentValidator::from("+-.");
    for (data, expected) in test_cases {
        let result: bool = validator.is_valid(data.as_bytes());
        assert_eq!(result, *expected, "data={}", *data);
    }
}
