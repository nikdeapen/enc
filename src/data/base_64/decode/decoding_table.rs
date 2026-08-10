use crate::base_64::Base64Encoder;
use crate::base_64::encode::EncodingTable;
use crate::base_64::table::Table;
use std::sync::Arc;

/// A base-64 decoding table.
pub type DecodingTable = Table<256>;

impl Table<256> {
    //! Special Tables

    /// The standard decoding table.
    const STANDARD: Self = Self::Static(&Self::create_custom_decoding_table(
        Base64Encoder::DEFAULT_V63,
        Base64Encoder::DEFAULT_V64,
    ));

    /// The URL-safe decoding table.
    const URL_SAFE: Self = Self::Static(&Self::create_custom_decoding_table(
        Base64Encoder::URL_SAFE_V63,
        Base64Encoder::URL_SAFE_V64,
    ));
}

impl Table<256> {
    //! Construction

    /// Creates a custom decoding table. Returns the static table if available.
    pub fn get_decoding_table(v63: u8, v64: u8) -> Self {
        match (v63, v64) {
            (Base64Encoder::DEFAULT_V63, Base64Encoder::DEFAULT_V64) => Self::STANDARD,
            (Base64Encoder::URL_SAFE_V63, Base64Encoder::URL_SAFE_V64) => Self::URL_SAFE,
            (v63, v64) => Self::Reference(Arc::new(Self::create_custom_decoding_table(v63, v64))),
        }
    }

    /// Creates a custom decoding table. (the inverse of the matching encoding table)
    const fn create_custom_decoding_table(v63: u8, v64: u8) -> [u8; 256] {
        let encoding: [u8; 64] = EncodingTable::create_custom_encoding_table(v63, v64);
        let mut t: [u8; 256] = [0xFF; 256];

        let mut i: usize = 0;
        while i < encoding.len() {
            t[encoding[i] as usize] = i as u8;
            i += 1;
        }

        t
    }
}

impl Default for Table<256> {
    fn default() -> Self {
        Self::STANDARD
    }
}

#[cfg(test)]
mod tests {
    use crate::base_64::Base64Encoder;
    use crate::base_64::decode::decoding_table::DecodingTable;

    #[test]
    fn fn_get_decoding_table() {
        let standard: DecodingTable = DecodingTable::get_decoding_table(
            Base64Encoder::DEFAULT_V63,
            Base64Encoder::DEFAULT_V64,
        );
        assert!(matches!(standard, DecodingTable::Static(_)));

        let url_safe: DecodingTable = DecodingTable::get_decoding_table(
            Base64Encoder::URL_SAFE_V63,
            Base64Encoder::URL_SAFE_V64,
        );
        assert!(matches!(url_safe, DecodingTable::Static(_)));

        let custom: DecodingTable = DecodingTable::get_decoding_table(b'!', b'@');
        assert!(matches!(custom, DecodingTable::Reference(_)));
    }

    #[test]
    fn fn_decoding_table() {
        let table: DecodingTable = DecodingTable::default();
        let t: &[u8; 256] = table.table();
        assert_eq!(t[b'A' as usize], 0);
        assert_eq!(t[b'Z' as usize], 25);
        assert_eq!(t[b'a' as usize], 26);
        assert_eq!(t[b'z' as usize], 51);
        assert_eq!(t[b'0' as usize], 52);
        assert_eq!(t[b'9' as usize], 61);
        assert_eq!(t[Base64Encoder::DEFAULT_V63 as usize], 62);
        assert_eq!(t[Base64Encoder::DEFAULT_V64 as usize], 63);
        assert_eq!(t[b'=' as usize], 0xFF);
        assert_eq!(t[b'!' as usize], 0xFF);
    }
}
