//! Hydrogen I18n // Multilanguage
//!
//! Search and parse the multilanguage files.

use std::{collections::HashMap, path::Path};

use crate::{language::Language, utils::search_files};

/// Manager for the languages files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Multilanguage {
    /// The cache of the languages.
    pub cache: HashMap<String, Language>,
    /// The default language.
    pub default_language: Language,
    /// The links between the languages.
    pub links: HashMap<String, String>,
}

impl Multilanguage {
    /// Create a new empty [`Multilanguage`] with only the default language.
    pub fn new(language: Language) -> Self {
        Self {
            cache: HashMap::new(),
            default_language: language,
            links: HashMap::new(),
        }
    }

    /// Add a language to the cache using the metadata code as the language name.
    pub fn add_language(&mut self, language: Language) -> bool {
        if let Some(name) = language.metadata().code().clone() {
            self.cache.insert(name.to_owned(), language);
            true
        } else {
            false
        }
    }

    /// Set the default language from the cache resolving the link if needed.
    pub fn set_default_language(&mut self, language: &str) {
        if let Some(language_obj) = self
            .links
            .get(language)
            .and_then(|link| self.cache.get(link))
        {
            self.default_language = language_obj.clone();
        } else {
            self.default_language = self
                .cache
                .get(language)
                .unwrap_or(&self.default_language)
                .clone();
        }
    }

    /// Load a language from a string interpreting the metadata to add it to the cache or the links.
    pub fn load_str(&mut self, s: &str) -> bool {
        if let Some(language) = Language::load_str(s) {
            if let Some(link) = language.metadata().link().map(ToOwned::to_owned) {
                if let Some(name) = language.metadata().code().map(ToOwned::to_owned) {
                    self.links.insert(name, link.to_owned());
                    return true;
                }
            } else {
                if let Some(language_name) = language.metadata().code().map(ToOwned::to_owned) {
                    self.cache.insert(language_name, language);
                    return true;
                }
            }
        }

        false
    }

    /// Load a language from a file interpreting the metadata to add it to the cache or the links, if the metadata code is not present, the file name will be used.
    pub fn load_file<P: AsRef<Path>>(&mut self, path: P) -> bool {
        if let Some(language) = Language::load_file(path.as_ref()) {
            if let Some(link) = language.metadata().link().map(ToOwned::to_owned) {
                if let Some(name) = language
                    .metadata()
                    .code()
                    .map(ToOwned::to_owned)
                    .or_else(|| get_language_from_file(path.as_ref()))
                {
                    self.links.insert(name, link.to_owned());
                    return true;
                }
            } else {
                if let Some(language_name) = language
                    .metadata()
                    .code()
                    .map(ToOwned::to_owned)
                    .or_else(|| get_language_from_file(path.as_ref()))
                {
                    self.cache.insert(language_name, language);
                    return true;
                }
            }
        }

        false
    }

    /// Load all the languages from a directory.
    pub fn load_dir<P: AsRef<Path>>(&mut self, path: P) {
        for file in search_files(path) {
            self.load_file(file);
        }
    }

    /// Cleanup the links removing the dangling ones.
    pub fn cleanup_links(&mut self) {
        let mut links = HashMap::new();
        for (name, link) in self.links.drain() {
            if self.cache.contains_key(&link) {
                links.insert(name, link);
            }
        }
        self.links = links;
    }

    /// Get the language from the cache, resolving the link if needed, or the default language if not found.
    pub fn get(&self, language: &str) -> &Language {
        if let Some(link) = self.links.get(language) {
            self.cache.get(link).unwrap_or(&self.default_language)
        } else {
            self.cache.get(language).unwrap_or(&self.default_language)
        }
    }

    /// Get the translation from the language in the cache, resolving the link if needed, or the default language if not found.
    pub fn translate(&self, language: &str, category: &str, key: &str) -> String {
        self.get(language)
            .get_translation(category, key)
            .cloned()
            .unwrap_or_else(|| self.default_language.translate(category, key))
    }

    /// Get the translation from the language in the cache, resolving the link if needed, or the default language if not found.
    pub fn translate_with<T: IntoIterator<Item = (String, String)>>(
        &self,
        language: &str,
        category: &str,
        key: &str,
        args: T,
    ) -> String {
        let mut translation = self
            .get(language)
            .get_translation(category, key)
            .cloned()
            .unwrap_or_else(|| self.default_language.translate(category, key));

        for (key, value) in args {
            translation = translation.replace(&format!("{{{}}}", key), &value);
        }

        translation
    }
}

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
impl Multilanguage {
    /// Load a language from a file asynchronously using [`tokio`], interpreting the metadata to add it to the cache or the links.
    pub async fn tokio_load_file<P: AsRef<Path>>(&mut self, path: P) -> bool {
        if let Some(language) = Language::tokio_load_file(path).await {
            if let Some(link) = language.metadata().link().map(ToOwned::to_owned) {
                if let Some(name) = language.metadata().code().map(ToOwned::to_owned) {
                    self.links.insert(name, link.to_owned());
                    return true;
                }
            } else {
                if let Some(language_name) = language.metadata().code().map(ToOwned::to_owned) {
                    self.cache.insert(language_name, language);
                    return true;
                }
            }
        }

        false
    }

    /// Load all the languages from a directory asynchronously using [`tokio`].
    pub async fn tokio_load_dir<P: AsRef<Path>>(&mut self, path: P) {
        for file in crate::utils::tokio_search_files(path).await {
            self.tokio_load_file(file).await;
        }
    }
}

/// Get the language from the file name.
pub fn get_language_from_file<P: AsRef<Path>>(path: P) -> Option<String> {
    path.as_ref()
        .file_stem()
        .and_then(|file_stem| file_stem.to_str())
        .map(|file_stem| file_stem.to_owned())
}
