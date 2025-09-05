#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]

const _: () = assert!(usize::BITS == 32 || usize::BITS == 64);

pub use error::*;
pub use util::*;

mod error;
mod util;

#[cfg(feature = "data")]
pub use data::*;
#[cfg(feature = "value")]
pub use value::*;

#[cfg(feature = "data")]
mod data;
#[cfg(feature = "value")]
mod value;

#[cfg(feature = "test")]
pub mod test;
