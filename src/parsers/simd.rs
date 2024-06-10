//! Hydrogen I18n's Language parser using [simd-json](https://docs.rs/simd-json/).

use crate::{Category, Error, Result};
use std::collections::HashMap;
use std::io::Read;

/// Deserializes a JSON reader into a language.
pub fn parse_from_reader<R: Read>(reader: R) -> Result<HashMap<String, Category>> {
    simd_json::from_reader(reader).map_err(Error::SimdJson)
}

/// Deserializes a JSON slice into a language.
pub fn parse_from_slice(slice: &mut [u8]) -> Result<HashMap<String, Category>> {
    simd_json::from_slice(slice).map_err(Error::SimdJson)
}

/// Deserializes a JSON string into a language.
pub fn parse_from_str(s: &mut str) -> Result<HashMap<String, Category>> {
    unsafe { simd_json::from_str(s).map_err(Error::SimdJson) }
}
