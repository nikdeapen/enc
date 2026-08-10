use crate::Error;
use crate::test::hex;

/// Tests the `Encoder` or `Decoder` functions.
pub fn test_coder<I, O, LF, SF, AF, VF>(
    test_cases: &[(I, O)],
    len_fn: LF,
    slice_fn: SF,
    append_fn: AF,
    vec_fn: VF,
) where
    I: AsRef<[u8]>,
    O: AsRef<[u8]>,
    LF: Fn(&[u8]) -> Result<usize, Error>,
    SF: Fn(&[u8], &mut [u8]) -> Result<usize, Error>,
    AF: Fn(&[u8], &mut Vec<u8>) -> Result<usize, Error>,
    VF: Fn(&[u8]) -> Result<Vec<u8>, Error>,
{
    const GUARD: u8 = 0xA5;

    for (input, expected) in test_cases {
        let input: &[u8] = input.as_ref();
        let expected: &[u8] = expected.as_ref();

        assert_eq!(
            len_fn(input).unwrap(),
            expected.len(),
            "input={}",
            hex(input)
        );

        let mut output: Vec<u8> = vec![GUARD; expected.len() + 1];
        let output: &mut [u8] = output.as_mut_slice();
        assert_eq!(
            slice_fn(input, output).unwrap(),
            expected.len(),
            "input={}",
            hex(input)
        );
        assert_eq!(expected, &output[..expected.len()], "input={}", hex(input));
        assert_eq!(output[expected.len()], GUARD, "input={}", hex(input));

        let mut output: Vec<u8> = Vec::default();
        assert_eq!(
            append_fn(input, &mut output).unwrap(),
            expected.len(),
            "input={}",
            hex(input)
        );
        assert_eq!(expected, output.as_slice(), "input={}", hex(input));

        let mut output: Vec<u8> = vec![GUARD; 2];
        assert_eq!(
            append_fn(input, &mut output).unwrap(),
            expected.len(),
            "input={}",
            hex(input)
        );
        assert_eq!(&output[..2], &[GUARD; 2], "input={}", hex(input));
        assert_eq!(expected, &output[2..], "input={}", hex(input));

        let output: Vec<u8> = vec_fn(input).unwrap();
        assert_eq!(expected, output.as_slice(), "input={}", hex(input));
    }
}
