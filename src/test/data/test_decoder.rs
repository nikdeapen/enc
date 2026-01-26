use crate::test::data::util::test_coder;
use crate::Decoder;

/// Tests the `decoder`.
pub fn test_decoder<E, I, O>(decoder: &E, test_cases: &[(I, O)])
where
    E: Decoder,
    I: AsRef<[u8]>,
    O: AsRef<[u8]>,
{
    test_coder(
        test_cases,
        |input| decoder.decoded_len(input),
        |input, output| decoder.decode_to_slice(input, output),
        |input, output| decoder.append_to_vec(input, output),
        |input| decoder.decode_as_vec(input),
    )
}
