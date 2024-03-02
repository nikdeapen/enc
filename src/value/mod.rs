pub use decode_from_read::*;
#[cfg(feature = "var-int")]
pub use decode_from_read_length_prefixed::*;
pub use decode_from_read_prefix::*;
pub use encode_to_slice::*;
pub use encode_to_write::*;
#[cfg(feature = "var-int")]
pub use encode_to_write_length_prefixed::*;
pub use encoded_len::*;

mod decode_from_read;
#[cfg(feature = "var-int")]
mod decode_from_read_length_prefixed;
mod decode_from_read_prefix;
mod encode_to_slice;
mod encode_to_write;
#[cfg(feature = "var-int")]
mod encode_to_write_length_prefixed;
mod encoded_len;

pub(crate) mod util;
