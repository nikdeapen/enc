use crate::base_64::Base64Encoder;
use std::sync::Arc;

/// A base-64 decoding table.
///
/// This implementation supports static and atomic reference counted tables. This strategy avoids
/// all allocation for the standard decoding table configurations and also avoids allocation when
/// cloning reference counted tables. The only allocation needed is when creating a new,
/// non-standard decoding table.
#[derive(Clone, Debug)]
pub enum DecodingTable {
    /// A static decoding table.
    Static(&'static [u8; 256]),

    /// An atomic reference counted decoding table.
    Reference(Arc<[u8; 256]>),
}

impl DecodingTable {
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

impl DecodingTable {
    //! Construction

    /// Creates a custom decoding table. Returns the static table if available.
    pub fn get_decoding_table(v63: u8, v64: u8) -> Self {
        match (v63, v64) {
            (Base64Encoder::DEFAULT_V63, Base64Encoder::DEFAULT_V64) => Self::STANDARD,
            (Base64Encoder::URL_SAFE_V63, Base64Encoder::URL_SAFE_V64) => Self::URL_SAFE,
            (v63, v64) => Self::Reference(Arc::new(Self::create_custom_decoding_table(v63, v64))),
        }
    }

    /// Creates a custom decoding table.
    const fn create_custom_decoding_table(v63: u8, v64: u8) -> [u8; 256] {
        let mut t: [u8; 256] = [0xFF; 256];

        t[b'A' as usize] = 0;
        t[b'B' as usize] = 1;
        t[b'C' as usize] = 2;
        t[b'D' as usize] = 3;
        t[b'E' as usize] = 4;
        t[b'F' as usize] = 5;
        t[b'G' as usize] = 6;
        t[b'H' as usize] = 7;
        t[b'I' as usize] = 8;
        t[b'J' as usize] = 9;
        t[b'K' as usize] = 10;
        t[b'L' as usize] = 11;
        t[b'M' as usize] = 12;
        t[b'N' as usize] = 13;
        t[b'O' as usize] = 14;
        t[b'P' as usize] = 15;
        t[b'Q' as usize] = 16;
        t[b'R' as usize] = 17;
        t[b'S' as usize] = 18;
        t[b'T' as usize] = 19;
        t[b'U' as usize] = 20;
        t[b'V' as usize] = 21;
        t[b'W' as usize] = 22;
        t[b'X' as usize] = 23;
        t[b'Y' as usize] = 24;
        t[b'Z' as usize] = 25;

        t[b'a' as usize] = 26;
        t[b'b' as usize] = 27;
        t[b'c' as usize] = 28;
        t[b'd' as usize] = 29;
        t[b'e' as usize] = 30;
        t[b'f' as usize] = 31;
        t[b'g' as usize] = 32;
        t[b'h' as usize] = 33;
        t[b'i' as usize] = 34;
        t[b'j' as usize] = 35;
        t[b'k' as usize] = 36;
        t[b'l' as usize] = 37;
        t[b'm' as usize] = 38;
        t[b'n' as usize] = 39;
        t[b'o' as usize] = 40;
        t[b'p' as usize] = 41;
        t[b'q' as usize] = 42;
        t[b'r' as usize] = 43;
        t[b's' as usize] = 44;
        t[b't' as usize] = 45;
        t[b'u' as usize] = 46;
        t[b'v' as usize] = 47;
        t[b'w' as usize] = 48;
        t[b'x' as usize] = 49;
        t[b'y' as usize] = 50;
        t[b'z' as usize] = 51;

        t[b'0' as usize] = 52;
        t[b'1' as usize] = 53;
        t[b'2' as usize] = 54;
        t[b'3' as usize] = 55;
        t[b'4' as usize] = 56;
        t[b'5' as usize] = 57;
        t[b'6' as usize] = 58;
        t[b'7' as usize] = 59;
        t[b'8' as usize] = 60;
        t[b'9' as usize] = 61;

        t[v63 as usize] = 62;
        t[v64 as usize] = 63;

        t
    }
}

impl Default for DecodingTable {
    fn default() -> Self {
        Self::STANDARD
    }
}

impl DecodingTable {
    //! Properties

    /// Gets the raw decoding table.
    pub fn decoding_table(&self) -> &[u8; 256] {
        match self {
            Self::Static(table) => table,
            Self::Reference(table) => table.as_ref(),
        }
    }
}

// todo -- test cases
