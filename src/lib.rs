#![doc=include_str!("../README.md")]

pub(crate) mod formats;
pub(crate) mod value;
pub(crate) mod value_trait;
pub use formats::*;
pub use value::*;
pub use value_trait::*;

#[doc(inline)]
#[allow(unused_imports)]
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use rusty_value_derive::*;
