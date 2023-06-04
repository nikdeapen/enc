use crate::percent::PercentEncoder;
use crate::StringEncoder;

#[test]
fn encode() {
    let test_cases: &[(&str, &str)] = &[
        ("", ""),
        ("azAZ09", "azAZ09"),
        ("+-.", "+-."),
        ("!@#$", "%21%40%23%24"),
        ("%", "%25"),
        (" ", "%20"),
    ];
    let encoder: PercentEncoder = PercentEncoder::from("+-.");
    for (data, expected) in test_cases {
        let result: String = encoder.encode_as_string(data.as_bytes()).unwrap();
        assert_eq!(result, *expected);
    }
}
