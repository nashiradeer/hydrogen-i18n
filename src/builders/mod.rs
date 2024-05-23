//! # Hydrogen I18n // Builders
//!
//! This module contains the builders for the [`I18n`] struct.

use std::{collections::HashMap, io::Read};

use crate::{Category, Error, Result};

mod sync;
pub use sync::I18nBuilder;

/// Re-export of the `serde_json` crate used by Hydrogen I18n.
pub use serde_json;

#[cfg(feature = "simd")]
#[cfg_attr(docsrs, doc(cfg(feature = "simd")))]
/// Re-export of the `simd_json` crate used by Hydrogen I18n.
pub use simd_json;

#[cfg(not(feature = "simd"))]
/// Deserializes a JSON reader into a language.
pub fn from_reader<R: Read>(reader: R) -> Result<HashMap<String, Category>> {
    serde_json::from_reader(reader).map_err(Error::Json)
}

#[cfg(not(feature = "simd"))]
/// Deserializes a JSON slice into a language.
pub fn from_slice<'a>(slice: &'a [u8]) -> Result<HashMap<String, Category>> {
    serde_json::from_slice(slice).map_err(Error::Json)
}

#[cfg(not(feature = "simd"))]
/// Deserializes a JSON string into a language.
pub fn from_str(s: &str) -> Result<HashMap<String, Category>> {
    serde_json::from_str(s).map_err(Error::Json)
}

#[cfg(feature = "simd")]
#[cfg_attr(docsrs, doc(cfg(feature = "simd")))]
/// Deserializes a JSON reader into a language.
pub fn from_reader<R: Read>(reader: R) -> Result<HashMap<String, Category>> {
    simd_json::from_reader(reader).map_err(Error::SimdJson)
}

#[cfg(feature = "simd")]
#[cfg_attr(docsrs, doc(cfg(feature = "simd")))]
/// Deserializes a JSON slice into a language.
pub fn from_slice<'a>(slice: &'a [u8]) -> Result<HashMap<String, Category>> {
    simd_json::from_slice(slice).map_err(Error::SimdJson)
}

#[cfg(feature = "simd")]
#[cfg_attr(docsrs, doc(cfg(feature = "simd")))]
/// Deserializes a JSON string into a language.
pub fn from_str(s: &str) -> Result<HashMap<String, Category>> {
    simd_json::from_str(s).map_err(Error::SimdJson)
}

fn resolve_translation<'a>(
    language: &'a HashMap<String, Category>,
    category: &str,
    key: &str,
) -> Option<&'a String> {
    language.get(category)?.get(key)
}
