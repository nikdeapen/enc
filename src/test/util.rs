/// Converts the `data` to a human-readable hex string.
///
/// # Example
/// &[255, 255, 255] -> "FFFF:FF"
pub fn hex(data: &[u8]) -> String {
    let mut s: String = String::with_capacity(data.len() * 3);

    for (i, b) in data.iter().enumerate() {
        if i != 0 && i != data.len() && i % 2 == 0 {
            s.push(':');
        }
        // todo -- HexEncoder
        s.push_str(&format!("{:02x}", b));
    }

    s
}
