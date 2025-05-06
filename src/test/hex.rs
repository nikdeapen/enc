use crate::hex::HexEncoder;

/// Converts the `data` to a human-readable hex string.
pub fn hex(data: &[u8]) -> String {
    let mut s: String = String::with_capacity(data.len() * 3);

    for (i, b) in data.iter().enumerate() {
        if i != 0 && i != data.len() && i % 2 == 0 {
            s.push(':');
        }
        let (a, b) = HexEncoder::UPPER.encode_chars(*b);
        s.push(a);
        s.push(b);
    }

    s
}
