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
//! Hydrogen I18n is part of [Hydrogen Framework](https://github.com/users/nashiradeer/projects/8), this means that this crate is designed to be used on Discord bots created using [serenity](https://crates.io/crates/serenity) and [twilight](https://crates.io/crates/twilight), but is sufficiently generic to be used by any other application that not use these Discord libraries or even are Discord bots.
//!
//! ## Donating
//!
//! Consider donating to make Hydrogen I18n development possible. You can donate thought Nashira Deer's [PayPal](https://www.paypal.com/donate/?business=QQGMTC3FQAJF6&no_recurring=0&item_name=Thanks+for+donating+for+me%2C+this+helps+me+a+lot+to+continue+developing+and+maintaining+my+projects.&currency_code=USD) or [GitHub Sponsor](https://github.com/sponsors/nashiradeer).
//!
//! ## Features
//!
//! - `serenity`: Enables functions that make it easy to use the library in Discord apps and bots built with [serenity](https://crates.io/crates/serenity).
//!
//! ## Credits
//!
//! Hydrogen I18n is a Nashira Deer's project licensed under the [MIT License](https://github.com/nashiradeer/hydrogen-i18n/blob/main/LICENSE.txt) and licensed under the [GNU Lesser General Public License v3](https://github.com/nashiradeer/hydrogen-i18n/blob/c00b016356dc9263571e6cc6ede87969bf31bf02/LICENSE.txt) until v1.0.1.

use std::{
    collections::{hash_map, HashMap},
    fmt::{self, Display, Formatter},
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
#[derive(Debug)]
pub enum Error {
    /// An error related to IO.
    Io(io::Error),

    /// An error related to JSON parsing.
    Json(serde_json::Error),

    /// The language was not found.
    LanguageNotFound(String),

    /// An error related to UTF-8 parsing.
    Utf8(std::str::Utf8Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "IO error: {}", error),
            Self::Json(error) => write!(f, "JSON error: {}", error),
            Self::LanguageNotFound(language) => {
                write!(f, "Language {} not found", language)
            }
            Self::Utf8(error) => write!(f, "UTF-8 error: {}", error),
        }
    }
}

/// A result with the error type of Hydrogen I18n.
pub type Result<T> = result::Result<T, Error>;

/// A single category containing translations as key-value pairs.
pub type Category = HashMap<String, String>;

/// A language containing categories as key-value pairs or a link to another language.
#[derive(Clone)]
pub enum Language {
    /// A link to another language.
    Link(String),

    /// A language containing categories as key-value pairs.
    Data(HashMap<String, Category>),
}

/// Translation manager, used to load and manage all languages in the memory.
#[derive(Clone, Default)]
pub struct I18n {
    /// All languages managed by this instance.
    pub languages: HashMap<String, Language>,

    /// The default language. Used as fallback when a language, category or key is not found.
    pub default: HashMap<String, Category>,
}

impl I18n {
    /// Creates a new instance of the manager, proving the default language and the languages that will be managed.
    pub fn new_with_default_and_languages(
        default: HashMap<String, Category>,
        languages: HashMap<String, Language>,
    ) -> Self {
        Self { languages, default }
    }

    /// Creates a new instance of the manager, proving the default language.
    pub fn new_with_default(default: HashMap<String, Category>) -> Self {
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

    /// Removes all the translations that are equal to the default translation.
    pub fn deduplicate(&self, language: &mut HashMap<String, Category>) {
        language.retain(|category_name, category| {
            category.retain(|key, value| {
                if let Some(default_translation) = self.translate_default_option(category_name, key)
                {
                    if default_translation == *value {
                        return false;
                    }
                }

                true
            });

            !category.is_empty()
        });
    }

    /// Loads a language or a link from a `&str` of Hydrogen I18n's JSON.
    ///
    /// If check_link is `true` and the language is a link, it will check if the language exists.
    pub fn from_str(
        &mut self,
        language: &str,
        data: &str,
        check_link: bool,
        deduplicate: bool,
    ) -> Result<()> {
        if let Some(link) = data.strip_prefix("_link:") {
            if check_link && !self.languages.contains_key(link) {
                return Err(Error::LanguageNotFound(link.to_owned()));
            }

            self.languages
                .insert(language.to_owned(), Language::Link(link.to_owned()));
        } else {
            let mut parsed_language = serde_json::from_str(data).map_err(Error::Json)?;

            if deduplicate {
                self.deduplicate(&mut parsed_language);
            }

            self.languages
                .insert(language.to_owned(), Language::Data(parsed_language));
        }

        Ok(())
    }

    /// Loads a language from a I/O stream of Hydrogen I18n's JSON.
    ///
    /// This function can't check if the language is a link, so it will always parse the data as a language.
    pub fn from_reader<R: Read>(
        &mut self,
        language: &str,
        reader: R,
        deduplicate: bool,
    ) -> serde_json::Result<()> {
        let mut parsed_language = serde_json::from_reader(reader)?;

        if deduplicate {
            self.deduplicate(&mut parsed_language);
        }

        self.languages
            .insert(language.to_owned(), Language::Data(parsed_language));
        Ok(())
    }

    /// Loads a language from a `&[u8]` of Hydrogen I18n's JSON.
    pub fn from_slice(
        &mut self,
        language: &str,
        data: &[u8],
        check_link: bool,
        deduplicate: bool,
    ) -> Result<()> {
        if let Some(link) = data.strip_prefix("_link:".as_bytes()) {
            let link_str = std::str::from_utf8(link).map_err(Error::Utf8)?;

            if check_link && !self.languages.contains_key(link_str) {
                return Err(Error::LanguageNotFound(link_str.to_owned()));
            }

            self.languages
                .insert(language.to_owned(), Language::Link(link_str.to_owned()));
        } else {
            let mut parsed_language = serde_json::from_slice(data).map_err(Error::Json)?;

            if deduplicate {
                self.deduplicate(&mut parsed_language);
            }

            self.languages
                .insert(language.to_owned(), Language::Data(parsed_language));
        }

        Ok(())
    }

    /// Loads a language from a [serde_json::Value] of Hydrogen I18n's JSON.
    pub fn from_value(
        &mut self,
        language: &str,
        data: serde_json::Value,
        deduplicate: bool,
    ) -> serde_json::Result<()> {
        let mut parsed_language = serde_json::from_value(data)?;

        if deduplicate {
            self.deduplicate(&mut parsed_language);
        }

        self.languages
            .insert(language.to_owned(), Language::Data(parsed_language));
        Ok(())
    }

    /// Sets the default language from the languages already loaded.
    ///
    /// This function will fail if the language is a link.
    ///
    /// If deduplicate is `true`, the language will be removed instead of cloned.
    ///
    /// Returns `true` if the language was found and set as default.
    pub fn set_default(&mut self, language: &str, deduplicate: bool) -> bool {
        let Some(Language::Data(language)) = ({
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
    pub fn from_file<P: AsRef<Path>>(
        &mut self,
        language: &str,
        path: P,
        deduplicate: bool,
    ) -> Result<()> {
        let file = File::open(path).map_err(Error::Io)?;
        let buffered_reader = BufReader::new(file);
        self.from_reader(language, buffered_reader, deduplicate)
            .map_err(Error::Json)
    }

    /// Loads all languages from a directory containing files of Hydrogen I18n's JSON, ignoring files that give errors.
    ///
    /// When loading a language, the file name will be used as the language name.
    ///
    /// All files loaded will be parsed as languages, ignoring links.
    pub fn from_dir<P: AsRef<Path>>(&mut self, path: P, deduplicate: bool) -> Result<()> {
        for entry in path.as_ref().read_dir().map_err(Error::Io)? {
            if let Ok(file) = entry {
                let path = file.path();
                if let Some(language) = path
                    .file_stem()
                    .map(|s| s.to_str().map(|f| f.to_owned()))
                    .flatten()
                {
                    _ = self.from_file(&language, path, deduplicate);
                }
            }
        }

        Ok(())
    }

    /// Loads all languages from a directory containing files of Hydrogen I18n's JSON, ignoring files that give errors.
    ///
    /// This function considers the file extension as your content type, *.json for languages and *.link for links.
    ///
    /// If check_link is `true` when loading a link, it will check if the language exists.
    pub fn from_dir_with_links<P: AsRef<Path>>(
        &mut self,
        path: P,
        check_link: bool,
        deduplicate: bool,
    ) -> Result<()> {
        for entry in path.as_ref().read_dir().map_err(Error::Io)? {
            if let Ok(file) = entry {
                let path = file.path();

                match file.path().extension().map(|s| s.to_str()).flatten() {
                    Some("json") => {
                        if let Some(language) = path
                            .file_stem()
                            .map(|s| s.to_str().map(|f| f.to_owned()))
                            .flatten()
                        {
                            _ = self.from_file(&language, path, deduplicate);
                        }
                    }
                    Some("link") => {
                        if let Some(language) = path
                            .file_stem()
                            .map(|s| s.to_str().map(|f| f.to_owned()))
                            .flatten()
                        {
                            let file = File::open(path).map_err(Error::Io)?;
                            let mut data = String::new();
                            let Ok(_) = file.take(16).read_to_string(&mut data) else {
                                continue;
                            };

                            if let Some(link) = data.strip_prefix("_link:") {
                                if check_link && !self.languages.contains_key(link) {
                                    continue;
                                }

                                self.languages
                                    .insert(language.to_owned(), Language::Link(link.to_owned()));
                            }
                        }
                    }
                    Some(_) | None => {}
                }
            }
        }

        Ok(())
    }

    /// Gets a language resolving link.
    ///
    /// Returns `None` if the language doesn't exist, if link isn't valid or if the link points to another link.
    pub fn get_language(&self, language: &str) -> Option<&HashMap<String, Category>> {
        match self.languages.get(language)? {
            Language::Link(link) => {
                let link = self.languages.get(link)?;
                match link {
                    Language::Link(_) => None,
                    Language::Data(language) => Some(language),
                }
            }
            Language::Data(language) => Some(language),
        }
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
        self.get_language(language)?
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
            i18n: self,
            category,
            key,
        }
    }

    /// Removes all the links that points to another link or to a language that doesn't exist.
    pub fn cleanup_links(&mut self) {
        let mut languages = HashMap::new();
        let mut link_languages = Vec::new();

        for (language_name, language) in self.languages.drain() {
            match language {
                Language::Link(link) => {
                    link_languages.push((language_name, link));
                }
                Language::Data(language) => {
                    languages.insert(language_name, Language::Data(language));
                }
            }
        }

        for (language_name, link) in link_languages {
            if let Some(Language::Data(_)) = languages.get(&link) {
                languages.insert(language_name, Language::Link(link));
            }
        }

        self.languages = languages;
    }
}

/// An iterator over all the translations of a category and key, ignores invalid links or links that points to another link.
#[derive(Clone)]
pub struct Iter<'a> {
    /// A iterator over all the languages managed by this instance.
    languages: hash_map::Iter<'a, String, Language>,

    /// A reference to the manager.
    i18n: &'a I18n,

    /// The category to get the translations.
    category: &'a str,

    /// The key to get the translations.
    key: &'a str,
}

impl Iterator for Iter<'_> {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (language_name, language) = match self.languages.next()? {
                (language_name, Language::Link(link)) => {
                    if let Some(language) = self.i18n.languages.get(link) {
                        match language {
                            Language::Link(_) => continue,
                            Language::Data(language) => (language_name, language),
                        }
                    } else {
                        continue;
                    }
                }
                (language_name, Language::Data(language)) => (language_name, language),
            };

            let Some(category) = language.get(self.category) else {
                continue;
            };

            let Some(translation) = category.get(self.key) else {
                continue;
            };

            return Some((language_name.to_owned(), translation.to_owned()));
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
