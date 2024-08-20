//! # Hydrogen I18n // Metadata
//!
//! Search and parse the metadata from the language files.

use serde::Deserialize;

/// Metadata for the language file.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Metadata {
    /// The language code.
    code: Option<String>,
    /// The language name in the language itself.
    name: Option<String>,
    /// Is some if this language is a link to another one.
    link: Option<String>,
}

impl Metadata {
    /// Get the language code.
    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }

    /// Get the language name.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Get the language link.
    pub fn link(&self) -> Option<&str> {
        self.link.as_deref()
    }

    /// Check if this language is a link to another one.
    pub fn is_link(&self) -> bool {
        self.link.is_some()
    }

    /// Check if this language is a language itself.
    pub fn is_language(&self) -> bool {
        self.link.is_none()
    }

    /// Get the language code and name.
    pub fn code_and_name(&self) -> Option<(&str, &str)> {
        Some((self.code()?, self.name()?))
    }
}
