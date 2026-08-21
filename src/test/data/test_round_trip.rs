use crate::test::hex;
use crate::{Decoder, Encoder};

/// Tests that the `decoder` recovers every input the `encoder` produces, for all data lengths up to `max_len`.
pub fn test_round_trip<E, D>(encoder: &E, decoder: &D, max_len: usize)
where
    E: Encoder,
    D: Decoder,
{
    let mut seed: u64 = 0x2545_F491_4F6C_DD1D;
    for len in 0..max_len {
        let data: Vec<u8> = (0..len).map(|_| next_byte(&mut seed)).collect();

        let encoded: Vec<u8> = encoder.encode_as_vec(&data).unwrap();
        let decoded: Vec<u8> = decoder.decode_as_vec(encoded.as_slice()).unwrap();

        assert_eq!(decoded, data, "len={len} data={}", hex(&data));
    }
}

/// Gets the next pseudo-random byte. (this crate has no dependencies, so it cannot use a real one)
fn next_byte(seed: &mut u64) -> u8 {
    *seed = seed
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    (*seed >> 33) as u8
}
