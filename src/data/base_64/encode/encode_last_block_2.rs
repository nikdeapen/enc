/// Encodes the last `block` with two bytes of data.
///
/// Returns the number of encoded bytes: (3 or 4).
#[inline(always)]
pub fn encode_last_block_2(
    table: &[u8; 64],
    padding: Option<u8>,
    block: &[u8],
    target: &mut [u8],
) -> usize {
    debug_assert_eq!(block.len(), 2);
    debug_assert!(target.len() >= 3);

    let a: u32 = block[0] as u32;
    let b: u32 = block[1] as u32;
    let bits: u32 = (a << 8) | b;

    let ai: usize = (bits >> 10) as usize;
    let bi: usize = ((bits >> 4) & 0x3F) as usize;
    let ci: usize = ((bits << 2) & 0x3F) as usize;

    target[0] = table[ai];
    target[1] = table[bi];
    target[2] = table[ci];

    if let Some(padding) = padding {
        debug_assert_eq!(target.len(), 4);
        target[3] = padding;
        4
    } else {
        debug_assert_eq!(target.len(), 3);
        3
    }
}
