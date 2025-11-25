use crate::base_64::Base64Encoder;
use std::sync::Arc;

/// A base-64 encoding table.
///
/// This implementation supports static and atomic reference counted tables. This strategy avoids
/// all allocation for the standard encoding table configurations and also avoids allocation when
/// cloning reference counted tables. The only allocation needed is when creating a new,
/// non-standard encoding table.
#[derive(Clone, Debug)]
pub enum EncodingTable {
    /// A static encoding table.
    Static(&'static [u8; 64]),

    /// An atomic reference counted encoding table.
    Reference(Arc<[u8; 64]>),
}

impl EncodingTable {
    //! Special Tables

    /// The standard encoding table.
    const STANDARD: Self = Self::Static(&Self::create_custom_encoding_table(
        Base64Encoder::DEFAULT_V63,
        Base64Encoder::DEFAULT_V64,
    ));

    /// The URL-safe encoding table.
    const URL_SAFE: Self = Self::Static(&Self::create_custom_encoding_table(
        Base64Encoder::URL_SAFE_V63,
        Base64Encoder::URL_SAFE_V64,
    ));
}

impl EncodingTable {
    //! Construction

    /// Creates a custom encoding table. Returns the static table if available.
    pub fn get_encoding_table(v63: u8, v64: u8) -> Self {
        match (v63, v64) {
            (Base64Encoder::DEFAULT_V63, Base64Encoder::DEFAULT_V64) => Self::STANDARD,
            (Base64Encoder::URL_SAFE_V63, Base64Encoder::URL_SAFE_V64) => Self::URL_SAFE,
            (v63, v64) => Self::Reference(Arc::new(Self::create_custom_encoding_table(v63, v64))),
        }
    }

    /// Creates a custom encoding table.
    const fn create_custom_encoding_table(v63: u8, v64: u8) -> [u8; 64] {
        let mut t: [u8; 64] = [0xFF; 64];

        let mut i: usize = 0;
        while i < 26 {
            t[i] = b'A' + i as u8;
            i += 1;
        }
        while i < 52 {
            t[i] = b'a' + (i - 26) as u8;
            i += 1;
        }
        while i < 62 {
            t[i] = b'0' + (i - 52) as u8;
            i += 1;
        }
        t[62] = v63;
        t[63] = v64;

        t
    }
}

impl Default for EncodingTable {
    fn default() -> Self {
        Self::STANDARD
    }
}

impl EncodingTable {
    //! Properties

    /// Gets the raw encoding table.
    pub fn encoding_table(&self) -> &[u8; 64] {
        match self {
            Self::Static(table) => table,
            Self::Reference(table) => table.as_ref(),
        }
    }
}

// todo -- test cases
