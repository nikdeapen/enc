pub use decode_from_read::*;
pub use decode_from_read_prefix::*;
pub use decode_from_slice::*;
pub use decode_from_slice_borrowed::*;
pub use encode_to_slice::*;
pub use encode_to_write::*;
pub use encoded_len::*;

mod decode_from_read;
mod decode_from_read_prefix;
mod decode_from_slice;
mod decode_from_slice_borrowed;
mod encode_to_slice;
mod encode_to_write;
mod encoded_len;

mod macros;

#[cfg(feature = "var-int")]
pub mod var_int;
