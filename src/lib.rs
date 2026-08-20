#![doc = include_str!("../README.md")]

#[cfg(feature = "data")]
pub use data::*;
pub use error::*;
pub use util::*;
#[cfg(feature = "value")]
pub use value::*;

#[cfg(feature = "data")]
mod data;
mod error;
mod util;
#[cfg(feature = "value")]
mod value;

#[cfg(feature = "dev")]
pub mod test;
