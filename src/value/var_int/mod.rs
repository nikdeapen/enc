use crate::write_stack_buf_impl;
pub use var_int_128::*;
pub use var_int_16::*;
pub use var_int_32::*;
pub use var_int_64::*;
pub use var_int_size::*;

mod var_int_128;
mod var_int_16;
mod var_int_32;
mod var_int_64;
mod var_int_size;

write_stack_buf_impl!(VarInt16, VarInt16::MAX_ENCODED_LEN);
write_stack_buf_impl!(VarInt32, VarInt32::MAX_ENCODED_LEN);
write_stack_buf_impl!(VarInt64, VarInt64::MAX_ENCODED_LEN);
write_stack_buf_impl!(VarInt128, VarInt128::MAX_ENCODED_LEN);
