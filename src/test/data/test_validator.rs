use crate::test::hex;
use crate::Validator;

/// Tests the `validator`.
pub fn test_validator<V, I>(validator: &V, test_cases: &[(I, bool)])
where
    V: Validator,
    I: AsRef<[u8]>,
{
    for (input, expected) in test_cases {
        let input: &[u8] = input.as_ref();

        let result: bool = validator.is_valid(input).unwrap();
        assert_eq!(result, *expected, "input={}", hex(input));
    }
}
