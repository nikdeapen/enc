#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]

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

#[cfg(feature = "test")]
pub mod test;
