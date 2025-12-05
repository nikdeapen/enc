/// Encodes the last `block` with one byte of data.
///
/// Returns the number of encoded bytes: (2 or 4).
///
/// # Safety
/// The `block` must be 1 byte in length.
/// The `target` must be 2 or 4 bytes in length, depending on `padding`.
#[inline(always)]
pub unsafe fn encode_last_block_1(
    table: &[u8; 64],
    padding: Option<u8>,
    block: &[u8],
    target: &mut [u8],
) -> usize {
    debug_assert_eq!(block.len(), 1);
    debug_assert!(target.len() >= 2);

    let bits: u32 = block[0] as u32;
    let ai: usize = (bits >> 2) as usize;
    let bi: usize = ((bits << 4) & 0x3F) as usize;

    target[0] = table[ai];
    target[1] = table[bi];

    if let Some(padding) = padding {
        debug_assert_eq!(target.len(), 4);
        target[2] = padding;
        target[3] = padding;
        4
    } else {
        debug_assert_eq!(target.len(), 2);
        2
    }
}

// todo -- test cases
