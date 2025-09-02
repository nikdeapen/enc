use crate::base_64::{Base64Decoder, Base64Encoder};

impl Base64Decoder {
    //! Constants

    /// The default 63rd value.
    pub const DEFAULT_V63: u8 = b'+';

    /// The default 64th value.
    pub const DEFAULT_V64: u8 = b'/';

    // The default padding.
    pub const DEFAULT_PADDING: Option<u8> = Some(b'=');

    /// The URL-safe 63rd value.
    pub const URL_SAFE_V63: u8 = b'-';

    /// The URL-safe 64th value.
    pub const URL_SAFE_V64: u8 = b'_';

    // The URL-safe padding.
    pub const URL_SAFE_PADDING: Option<u8> = Self::DEFAULT_PADDING;
}

impl Base64Encoder {
    //! Constants

    /// The default 63rd value.
    pub const DEFAULT_V63: u8 = Base64Decoder::DEFAULT_V63;

    /// The default 64th value.
    pub const DEFAULT_V64: u8 = Base64Decoder::DEFAULT_V64;

    // The default padding.
    pub const DEFAULT_PADDING: Option<u8> = Base64Decoder::DEFAULT_PADDING;

    /// The URL-safe 63rd value.
    pub const URL_SAFE_V63: u8 = Base64Decoder::URL_SAFE_V63;

    /// The URL-safe 64th value.
    pub const URL_SAFE_V64: u8 = Base64Decoder::URL_SAFE_V64;

    // The URL-safe padding.
    pub const URL_SAFE_PADDING: Option<u8> = Base64Decoder::URL_SAFE_PADDING;
}
