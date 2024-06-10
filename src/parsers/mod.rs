//! Contains wrappers for the different JSON parsers that are supported by the library.
//!
//! By default, the library uses [serde_json](https://docs.serde.rs/serde_json/), but you can enable the `simd` feature to use [simd-json](https://docs.rs/simd-json/).

pub mod serde;

#[cfg(not(feature = "simd"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "simd"))))]
pub use serde::*;

#[cfg(feature = "simd")]
#[cfg_attr(docsrs, doc(cfg(feature = "simd")))]
pub mod simd;

#[cfg(feature = "simd")]
#[cfg_attr(docsrs, doc(cfg(feature = "simd")))]
pub use simd::*;
