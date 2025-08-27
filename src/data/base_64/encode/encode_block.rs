/// Encodes a single full block of data.
///
/// This will encode 3 `data` bytes into 4 `target` bytes.
pub fn encode_block(table: &[u8; 64], data: &[u8], target: &mut [u8]) {
    debug_assert!(data.len() >= 3);
    debug_assert!(target.len() >= 4);

    let a: u32 = data[0] as u32;
    let b: u32 = data[1] as u32;
    let c: u32 = data[2] as u32;
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
