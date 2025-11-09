/// Encodes the full `block` of data.
///
/// This will encode 3 `block` bytes into 4 `target` bytes.
pub fn encode_block(table: &[u8; 64], block: &[u8], target: &mut [u8]) {
    debug_assert!(block.len() >= 3);
    debug_assert!(target.len() >= 4);

    let a: u32 = block[0] as u32;
    let b: u32 = block[1] as u32;
    let c: u32 = block[2] as u32;

    let bits: u32 = (a << 16) | (b << 8) | c;

    let ai: usize = (bits >> 18) as usize;
    let bi: usize = ((bits >> 12) & 0x3F) as usize;
    let ci: usize = ((bits >> 6) & 0x3F) as usize;
    let di: usize = (bits & 0x3F) as usize;

    target[0] = table[ai];
    target[1] = table[bi];
    target[2] = table[ci];
    target[3] = table[di];
}
