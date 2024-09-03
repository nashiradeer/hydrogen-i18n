//! # Hydrogen I18n // Language
//!
//! Search and parse the language files.

use std::{collections::HashMap, fs::read_to_string, path::Path};

use crate::metadata::Metadata;

/// Internal data for the [ `Language` ] struct.
pub type LanguageData = HashMap<String, HashMap<String, String>>;

/// A single language file with its metadata.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Language {
    /// Internal data for the language.
    data: LanguageData,
    /// Metadata for the language.
    metadata: Metadata,
}

impl Language {
    /// Create a new empty language.
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse the metadata from a string.
    pub fn load_str(s: &str) -> Option<Self> {
        Some(From::<LanguageData>::from(toml::from_str(s).ok()?))
    }

    /// Parse the metadata from a file.
    pub fn load_file<P: AsRef<Path>>(path: P) -> Option<Self> {
        Self::load_str(&read_to_string(path).ok()?)
    }

    /// Get a reference to a category.
    pub fn get(&self, category: &str) -> Option<&HashMap<String, String>> {
        self.data.get(category)
    }

    /// Get the translation value.
    pub fn get_translation(&self, category: &str, key: &str) -> Option<&String> {
        self.get(category)?.get(key)
    }

    /// Get the translation value or return the category and key.
    pub fn translate(&self, category: &str, key: &str) -> String {
        self.get_translation(category, key)
            .cloned()
            .unwrap_or(format!("{category}.{key}"))
    }

    /// Get the translation value with arguments.
    pub fn translate_with<T: IntoIterator<Item = (String, String)>>(
        &self,
        category: &str,
        key: &str,
        args: T,
    ) -> String {
        let mut translation = self.translate(category, key);

        for (key, value) in args {
            translation = translation.replace(&format!("{{{}}}", key), &value);
        }

        translation
    }

    /// Get the language data.
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
impl Language {
    /// Parse the metadata from a file asynchronously using [`tokio`].
    pub async fn tokio_load_file<P: AsRef<Path>>(path: P) -> Option<Self> {
        let file_content = tokio::fs::read_to_string(path).await.ok()?;

        tokio::task::spawn_blocking(move || Self::load_str(&file_content))
            .await
            .ok()?
    }
}

impl From<LanguageData> for Language {
    fn from(mut data: LanguageData) -> Self {
        Self {
            metadata: data.remove("_metadata").map(From::from).unwrap_or_default(),
            data,
        }
    }
}
