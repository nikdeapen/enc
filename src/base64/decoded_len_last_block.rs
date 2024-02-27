use crate::base64::remove_padding_last_block::remove_padding_last_block;

/// Gets the length of the decoded last block.
#[inline(always)]
pub unsafe fn decoded_length_last_block(padding: Option<u8>, data: &[u8]) -> usize {
    debug_assert!(data.len() <= 4);

    let data: &[u8] = remove_padding_last_block(data, padding);
    match data.len() {
        0 => 0, // the data is empty
        1 => 1, // invalid, we assume two more 0 bits
        2 => 1, // may be invalid, we ignore the last 4 bits
        3 => 2, // may be invalid, we ignore the last 2 bits
        4 => 3,
        _ => unreachable!(),
    }
}
