//! # Hydrogen I18n // Metadata
//!
//! Search and parse the metadata from the language files.

use std::{
    collections::HashMap,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use serde::Deserialize;

use crate::utils::search_files;

/// Metadata for the language file.
#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
pub struct Metadata {
    /// The language code.
    code: Option<String>,
    /// The language name in the language itself.
    name: Option<String>,
    /// Is some if this language is a link to another one.
    link: Option<String>,
}

impl Metadata {
    /// Create a new empty metadata.
    pub fn new() -> Self {
        Self::default()
    }

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

/// Metadata builder.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MetadataBuilder {
    #[serde(rename = "_metadata")]
    /// The metadata found inside the language file.
    metadata: Option<Metadata>,

    #[serde(skip)]
    /// The path to the language file.
    file: Option<PathBuf>,
}

impl MetadataBuilder {
    /// Build the metadata from the builder.
    pub fn build(self) -> Option<Metadata> {
        self.metadata
    }

    /// Get the metadata from the builder.
    pub fn metadata(&self) -> Option<&Metadata> {
        self.metadata.as_ref()
    }

    /// Get the path to the language file.
    pub fn file(&self) -> Option<&Path> {
        self.file.as_deref()
    }

    /// Parse the metadata from a string.
    pub fn load_str(s: &str) -> Option<Self> {
        toml::from_str(s).ok()
    }

    /// Parse the metadata from a file.
    pub fn load_file<P: AsRef<Path>>(path: P) -> Option<Self> {
        Self::load_str(&read_to_string(path.as_ref()).ok()?).map(|mut m| {
            m.file = Some(path.as_ref().to_path_buf());
            m
        })
    }

    /// Parse the metadata from all the files in a directory.
    pub fn load_dir<P: AsRef<Path>>(path: P) -> Vec<Self> {
        let mut metadatas = search_files(path)
            .into_iter()
            .filter_map(Self::load_file)
            .collect::<Vec<_>>();

        metadatas.shrink_to_fit();
        metadatas
    }
}

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
impl MetadataBuilder {
    /// Parse the metadata from a file asynchronously using [`tokio`].
    pub async fn tokio_load_file<P: AsRef<Path>>(path: P) -> Option<Self> {
        let file_content = tokio::fs::read_to_string(path.as_ref()).await.ok()?;

        tokio::task::spawn_blocking(move || Self::load_str(&file_content))
            .await
            .ok()?
            .map(|mut m: MetadataBuilder| {
                m.file = Some(path.as_ref().to_path_buf());
                m
            })
    }

    /// Parse the metadata from all the files in a directory asynchronously using [`tokio`].
    pub async fn tokio_load_dir<P: AsRef<Path>>(path: P) -> Vec<Self> {
        let files = crate::utils::tokio_search_files(path).await;
        let mut metadata = Vec::with_capacity(files.len());

        for file in &files {
            if let Some(m) = Self::tokio_load_file(file).await {
                metadata.push(m);
            }
        }

        metadata.shrink_to_fit();
        metadata
    }
}

impl From<HashMap<String, String>> for Metadata {
    fn from(map: HashMap<String, String>) -> Self {
        Self {
            code: map.get("code").cloned(),
            name: map.get("name").cloned(),
            link: map.get("link").cloned(),
        }
    }
}
