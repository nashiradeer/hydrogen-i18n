//! Hydrogen I18n's Language parser using [serde_json](https://docs.serde.rs/serde_json/).

use crate::{Category, Error, Result};
use std::{collections::HashMap, io::Read};

/// Deserializes a JSON reader into a language.
pub fn parse_from_reader<R: Read>(reader: R) -> Result<HashMap<String, Category>> {
    serde_json::from_reader(reader).map_err(Error::Json)
}

/// Deserializes a JSON slice into a language.
pub fn parse_from_slice(slice: &mut [u8]) -> Result<HashMap<String, Category>> {
    serde_json::from_slice(slice).map_err(Error::Json)
}

/// Deserializes a JSON string into a language.
pub fn parse_from_str(s: &mut str) -> Result<HashMap<String, Category>> {
    serde_json::from_str(s).map_err(Error::Json)
}
