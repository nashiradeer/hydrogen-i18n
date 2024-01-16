#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # Nashira Deer // Hydrogen I18n
//!
//! Translation utilities for server-side applications that need to deals with different languages on the same time.
//!
//! [![PayPal](https://img.shields.io/badge/Paypal-003087?style=for-the-badge&logo=paypal&logoColor=%23fff)](https://www.paypal.com/donate/?business=QQGMTC3FQAJF6&no_recurring=0&item_name=Thanks+for+donating+for+me%2C+this+helps+me+a+lot+to+continue+developing+and+maintaining+my+projects.&currency_code=USD)
//! [![GitHub Sponsor](https://img.shields.io/badge/GitHub%20Sponsor-181717?style=for-the-badge&logo=github&logoColor=%23fff)](https://github.com/sponsors/nashiradeer)
//! [![Crates.io](https://img.shields.io/crates/v/hydrogen-i18n?style=for-the-badge&logo=rust&logoColor=%23fff&label=Crates.io&labelColor=%23000&color=%23000)](https://crates.io/crates/hydrogen-i18n)
//! [![docs.rs](https://img.shields.io/docsrs/hydrogen-i18n?style=for-the-badge&logo=docsdotrs&logoColor=%23fff&label=Docs.rs&labelColor=%23000&color=%23000)](https://docs.rs/hydrogen-i18n/)
//!
//! Server-side applications that deal directly with users will need to be prepared to deal with different languages too, so Hydrogen I18n comes with utilities focused to make this task easier, loading and managing all the languages supported by the application in the memory, avoiding unnecessary disk and CPU usage.
//!
//! ## Donating
//!
//! Consider donating to make Hydrogen I18n development possible. You can donate thought Nashira Deer's [PayPal](https://www.paypal.com/donate/?business=QQGMTC3FQAJF6&no_recurring=0&item_name=Thanks+for+donating+for+me%2C+this+helps+me+a+lot+to+continue+developing+and+maintaining+my+projects.&currency_code=USD) or [GitHub Sponsor](https://github.com/sponsors/nashiradeer).
//!
//! ## Features
//!
//! - `serenity`: Enables functions that make it easy to use the library in Discord apps and bots built with Serenity.
//!
//! ## Credits
//!
//! Hydrogen I18n is a Nashira Deer's project licensed under the [MIT License](https://github.com/nashiradeer/hydrogen-i18n/blob/main/LICENSE.txt) and licensed under the [GNU Lesser General Public License v3](https://github.com/nashiradeer/hydrogen-i18n/blob/c00b016356dc9263571e6cc6ede87969bf31bf02/LICENSE.txt) until v1.0.1.

use std::{
    collections::{hash_map, HashMap},
    fs::File,
    io::{self, BufReader, Read},
    path::Path,
    result,
};

/// Re-export of the `serde_json` crate used by Hydrogen I18n.
pub use serde_json;

#[cfg(feature = "serenity")]
use serenity::builder::{CreateCommand, CreateCommandOption};

/// Groups all the errors that can be returned by Hydrogen I18n.
pub enum Error {
    /// An error related to IO.
    Io(io::Error),
    /// An error related to JSON parsing.
    Json(serde_json::Error),
}

/// A result with the error type of Hydrogen I18n.
pub type Result<T> = result::Result<T, Error>;

/// A single category containing translations as key-value pairs.
pub type Category = HashMap<String, String>;
/// A single language made of categories.
pub type Language = HashMap<String, Category>;

/// Translation manager, used to load and manage all languages in the memory.
#[derive(Clone, Default)]
pub struct I18n {
    /// All languages managed by this instance.
    pub languages: HashMap<String, Language>,
    /// The default language. Used as fallback when a language, category or key is not found.
    pub default: Language,
}

impl I18n {
    /// Creates a new instance of the manager, proving the default language and the languages that will be managed.
    pub fn new_with_default_and_languages(
        default: Language,
        languages: HashMap<String, Language>,
    ) -> Self {
        Self { languages, default }
    }

    /// Creates a new instance of the manager, proving the default language.
    pub fn new_with_default(default: Language) -> Self {
        Self::new_with_default_and_languages(default, HashMap::new())
    }

    /// Creates a new instance of the manager, proving the languages that will be managed.
    pub fn new_with_languages(languages: HashMap<String, Language>) -> Self {
        Self::new_with_default_and_languages(HashMap::new(), languages)
    }

    /// Creates a new instance of the manager.
    pub fn new() -> Self {
        Self::new_with_default_and_languages(HashMap::new(), HashMap::new())
    }

    /// Loads a language from a `&str` of Hydrogen I18n's JSON.
    pub fn from_str(&mut self, language: &str, data: &str) -> serde_json::Result<()> {
        let parsed_language = serde_json::from_str(data)?;
        self.languages.insert(language.to_owned(), parsed_language);
        Ok(())
    }

    /// Loads a language from a I/O stream of Hydrogen I18n's JSON.
    pub fn from_reader<R: Read>(&mut self, language: &str, reader: R) -> serde_json::Result<()> {
        let parsed_language = serde_json::from_reader(reader)?;
        self.languages.insert(language.to_owned(), parsed_language);
        Ok(())
    }

    /// Loads a language from a `&[u8]` of Hydrogen I18n's JSON.
    pub fn from_slice(&mut self, language: &str, data: &[u8]) -> serde_json::Result<()> {
        let parsed_language = serde_json::from_slice(data)?;
        self.languages.insert(language.to_owned(), parsed_language);
        Ok(())
    }

    /// Loads a language from a [serde_json::Value] of Hydrogen I18n's JSON.
    pub fn from_value(
        &mut self,
        language: &str,
        data: serde_json::Value,
    ) -> serde_json::Result<()> {
        let parsed_language = serde_json::from_value(data)?;
        self.languages.insert(language.to_owned(), parsed_language);
        Ok(())
    }

    /// Sets the default language from the languages already loaded.
    ///
    /// If deduplicate is `true`, the language will be removed instead of cloned.
    ///
    /// Returns `true` if the language was found and set as default.
    pub fn set_default(&mut self, language: &str, deduplicate: bool) -> bool {
        let Some(language) = ({
            if deduplicate {
                self.languages.remove(language)
            } else {
                self.languages.get(language).cloned()
            }
        }) else {
            return false;
        };

        self.default = language;
        true
    }

    /// Loads a language from a file of Hydrogen I18n's JSON.
    pub fn from_file<P: AsRef<Path>>(&mut self, language: &str, path: P) -> Result<()> {
        let file = File::open(path).map_err(Error::Io)?;
        let buffered_reader = BufReader::new(file);
        self.from_reader(language, buffered_reader)
            .map_err(Error::Json)
    }

    /// Loads all languages from a directory containing files of Hydrogen I18n's JSON, ignoring files that give errors.
    ///
    /// When loading a language, the file name will be used as the language name.
    pub fn from_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        for entry in path.as_ref().read_dir().map_err(Error::Io)? {
            if let Ok(file) = entry {
                let path = file.path();
                if let Some(language) = path
                    .file_stem()
                    .map(|s| s.to_str().map(|f| f.to_owned()))
                    .flatten()
                {
                    _ = self.from_file(&language, path);
                }
            }
        }

        Ok(())
    }

    /// Gets the translation for category and key using the default language.
    pub fn translate_default_option(&self, category: &str, key: &str) -> Option<String> {
        self.default.get(category)?.get(key).map(|f| f.clone())
    }

    /// Gets the translation for category and key using the default language, falling back to the format `category.key`.
    pub fn translate_default(&self, category: &str, key: &str) -> String {
        self.translate_default_option(category, key)
            .unwrap_or(format!("{}.{}", category, key))
    }

    /// Gets the translation for category and key using the specified language.
    pub fn translate_option(&self, language: &str, category: &str, key: &str) -> Option<String> {
        self.languages
            .get(language)?
            .get(category)?
            .get(key)
            .map(|f| f.clone())
    }

    /// Gets the translation for category and key using the specified language, falling back to the default language.
    pub fn translate(&self, language: &str, category: &str, key: &str) -> String {
        self.translate_option(language, category, key)
            .unwrap_or(self.translate_default(category, key))
    }

    /// Gets all the translations of a category and key from the languages managed by this instance.
    pub fn translate_all<'a>(&'a self, category: &'a str, key: &'a str) -> Iter<'_> {
        Iter {
            languages: self.languages.iter(),
            category,
            key,
        }
    }
}

/// An iterator over all the translations of a category and key.
#[derive(Clone)]
pub struct Iter<'a> {
    languages: hash_map::Iter<'a, String, Language>,
    category: &'a str,
    key: &'a str,
}

impl Iterator for Iter<'_> {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (language, language_map) = self.languages.next()?;

            let Some(category_map) = language_map.get(self.category) else {
                continue;
            };

            let Some(translation) = category_map.get(self.key) else {
                continue;
            };

            return Some((language.clone(), translation.clone()));
        }
    }
}

#[cfg(feature = "serenity")]
impl I18n {
    /// Inserts all the translations of a category and key from the languages managed by this instance into a [CreateCommand] as localized names.
    pub fn serenity_command_name(
        &self,
        category: &str,
        key: &str,
        mut command: CreateCommand,
    ) -> CreateCommand {
        for (language, translation) in self.translate_all(category, key) {
            command = command.name_localized(language, translation);
        }

        command
    }

    /// Inserts all the translations of a category and key from the languages managed by this instance into a [CreateCommand] as localized descriptions.
    pub fn serenity_command_description(
        &self,
        category: &str,
        key: &str,
        mut command: CreateCommand,
    ) -> CreateCommand {
        for (language, translation) in self.translate_all(category, key) {
            command = command.description_localized(language, translation);
        }

        command
    }

    /// Inserts all the translations of a category and key from the languages managed by this instance into a [CreateCommandOption] as localized names.
    pub fn serenity_command_option_name(
        &self,
        category: &str,
        key: &str,
        mut command_option: CreateCommandOption,
    ) -> CreateCommandOption {
        for (language, translation) in self.translate_all(category, key) {
            command_option = command_option.name_localized(language, translation);
        }

        command_option
    }

    /// Inserts all the translations of a category and key from the languages managed by this instance into a [CreateCommandOption] as localized descriptions.
    pub fn serenity_command_option_description(
        &self,
        category: &str,
        key: &str,
        mut command_option: CreateCommandOption,
    ) -> CreateCommandOption {
        for (language, translation) in self.translate_all(category, key) {
            command_option = command_option.description_localized(language, translation);
        }

        command_option
    }
}
