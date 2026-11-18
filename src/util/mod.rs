#[cfg(feature = "data")]
pub(crate) use append::*;
#[cfg(feature = "value")]
pub(crate) use append_unchecked::*;
pub use read::*;

#[cfg(feature = "data")]
mod append;
#[cfg(feature = "value")]
mod append_unchecked;
mod read;
