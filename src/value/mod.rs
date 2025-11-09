pub use decode_from_read::*;
pub use decode_from_read_prefix::*;
pub use encode_to_slice::*;
pub use encode_to_write::*;
pub use encoded_len::*;

mod decode_from_read;
mod decode_from_read_prefix;
mod encode_to_slice;
mod encode_to_write;
mod encoded_len;

#[cfg(feature = "var-int")]
pub mod var_int;

mod impls;
