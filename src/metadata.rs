//! # Hydrogen I18n // Metadata
//!
//! Search and parse the metadata from the language files.

use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use serde::Deserialize;

use crate::utils::search_files;

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
    pub fn from_str(s: &str) -> Option<Self> {
        toml::from_str(s).ok()
    }

    /// Parse the metadata from a file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Option<Self> {
        toml::from_str(&read_to_string(path.as_ref()).ok()?)
            .ok()
            .map(|mut m: MetadataBuilder| {
                m.file = Some(path.as_ref().to_path_buf());
                m
            })
    }

    /// Parse the metadata from all the files in a directory.
    pub fn from_dir<P: AsRef<Path>>(path: P) -> Vec<Option<Self>> {
        search_files(path)
            .iter()
            .map(|path| Self::from_file(path))
            .collect()
    }
}

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
impl MetadataBuilder {
    /// Parse the metadata from a file asynchronously using [`tokio`].
    pub async fn tokio_from_file<P: AsRef<Path>>(path: P) -> Option<Self> {
        let file_content = tokio::fs::read_to_string(path.as_ref()).await.ok()?;

        tokio::task::spawn_blocking(move || Self::from_str(&file_content))
            .await
            .ok()?
            .map(|mut m: MetadataBuilder| {
                m.file = Some(path.as_ref().to_path_buf());
                m
            })
    }

    /// Parse the metadata from all the files in a directory asynchronously using [`tokio`].
    pub async fn tokio_from_dir<P: AsRef<Path>>(path: P) -> Vec<Option<Self>> {
        let files = crate::utils::tokio_search_files(path).await;
        let mut metadata = Vec::with_capacity(files.len());

        for file in &files {
            let file = file.clone();
            metadata.push(Self::tokio_from_file(file).await);
        }

        metadata.shrink_to_fit();
        metadata
    }
}
