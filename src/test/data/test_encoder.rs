use crate::test::data::util::test_coder;
use crate::test::hex;
use crate::{Encoder, StringEncoder};

/// Tests the `encoder`.
pub fn test_encoder<E, I, O>(encoder: &E, test_cases: &[(I, O)])
where
    E: Encoder,
    I: AsRef<[u8]>,
    O: AsRef<[u8]>,
{
    test_coder(
        test_cases,
        |input| encoder.encoded_len(input),
        |input, output| encoder.encode_to_slice(input, output),
        |input, output| encoder.append_to_vec(input, output),
        |input| encoder.encode_as_vec(input),
    )
}

/// Tests the string `encoder`.
pub fn test_string_encoder<E, I, O>(encoder: &E, test_cases: &[(I, O)])
where
    E: StringEncoder,
    I: AsRef<[u8]>,
    O: AsRef<str> + AsRef<[u8]>,
{
    test_encoder(encoder, test_cases);

    for (input, expected) in test_cases {
        let input: &[u8] = input.as_ref();
        let expected: &str = expected.as_ref();

        let mut output: String = String::default();
        encoder.append_to_string(input, &mut output).unwrap();
        assert_eq!(expected, output, "input={}", hex(input));

        let output: String = encoder.encode_as_string(input).unwrap();
        assert_eq!(expected, output, "input={}", hex(input));
    }
}
