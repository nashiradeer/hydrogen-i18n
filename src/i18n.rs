//! Translation manager, used to load and manage all languages in the memory.

use std::{
    collections::{hash_map, HashMap},
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

#[allow(deprecated)]
use crate::{builders, deduplicate_language, Category, Error, Language, Result};

#[cfg(feature = "serenity")]
use serenity::builder::{CreateCommand, CreateCommandOption};

#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;

/// Translation manager, used to load and manage all languages in the memory.
#[derive(Clone, Default)]
pub struct I18n {
    /// All languages managed by this instance.
    pub languages: HashMap<String, Language>,

    /// The default language. Used as fallback when a language, category or key is not found.
    pub default: HashMap<String, Category>,
}

impl I18n {
    /// Creates a new instance of the manager using the builder pattern.
    pub fn builder() -> builders::I18nBuilder {
        Default::default()
    }

    #[cfg(feature = "tokio")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
    /// Creates a new instance of the manager using the builder pattern with Tokio support.
    pub fn tokio_builder() -> builders::TokioI18nBuilder {
        Default::default()
    }

    /// Creates a new instance of the manager, proving the default language and the languages that will be managed.
    #[deprecated(since = "2.2.0", note = "Use a builder instead")]
    pub fn new_with_default_and_languages(
        default: HashMap<String, Category>,
        languages: HashMap<String, Language>,
    ) -> Self {
        Self { languages, default }
    }

    /// Creates a new instance of the manager, proving the default language.
    #[deprecated(since = "2.2.0", note = "Use a builder instead")]
    pub fn new_with_default(default: HashMap<String, Category>) -> Self {
        #[allow(deprecated)]
        Self::new_with_default_and_languages(default, HashMap::new())
    }

    /// Creates a new instance of the manager, proving the languages that will be managed.
    #[deprecated(since = "2.2.0", note = "Use a builder instead")]
    pub fn new_with_languages(languages: HashMap<String, Language>) -> Self {
        #[allow(deprecated)]
        Self::new_with_default_and_languages(HashMap::new(), languages)
    }

    /// Creates a new instance of the manager.
    #[deprecated(since = "2.2.0", note = "Use a builder instead")]
    pub fn new() -> Self {
        #[allow(deprecated)]
        Self::new_with_default_and_languages(HashMap::new(), HashMap::new())
    }

    /// Removes all the translations that are equal to the default translation.
    #[deprecated(since = "2.2.0", note = "Use a builder instead")]
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

    /// Removes all the translations that are equal to the default translation from all languages managed by this instance.
    #[deprecated(since = "2.2.0", note = "Use a builder instead")]
    pub fn deduplicate_all(&mut self) {
        #[allow(deprecated)]
        for language in self.languages.values_mut() {
            match language {
                Language::Link(_) => {}
                Language::Data(language) => deduplicate_language(&self.default, language),
            }
        }
    }

    /// Loads a language or a link from a `&str` of Hydrogen I18n's JSON.
    ///
    /// ## Arguments
    ///
    /// If check_link is `true` and the language is a link, it will check if the language exists.
    ///
    /// If deduplicate is `true`, translations and categories equal to the default language will be removed.
    #[deprecated(since = "2.2.0", note = "Use a builder instead")]
    pub fn from_str(
        &mut self,
        language: &str,
        data: &str,
        check_link: bool,
        deduplicate: bool,
    ) -> Result<()> {
        #[allow(deprecated)]
        if let Some(link) = data.strip_prefix("_link:") {
            if check_link && !self.languages.contains_key(link) {
                return Err(Error::LanguageNotFound(link.to_owned()));
            }

            self.languages
                .insert(language.to_owned(), Language::Link(link.to_owned()));
        } else {
            #[cfg(not(feature = "simd"))]
            let mut parsed_language = serde_json::from_str(data).map_err(Error::Json)?;

            #[cfg(feature = "simd")]
            let mut parsed_language = unsafe {
                let mut data_cloned = data.to_owned();
                simd_json::from_str(&mut data_cloned).map_err(Error::SimdJson)
            }?;

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
    /// ## Arguments
    ///
    /// If deduplicate is `true`, translations and categories equal to the default language will be removed.
    ///
    /// ## Notes
    ///
    /// This function can't check if the language is a link, so it will always parse the data as a language.
    #[deprecated(since = "2.2.0", note = "Use a builder instead")]
    pub fn from_reader<R: Read>(
        &mut self,
        language: &str,
        reader: R,
        deduplicate: bool,
    ) -> Result<()> {
        #[cfg(not(feature = "simd"))]
        let mut parsed_language = serde_json::from_reader(reader).map_err(Error::Json)?;

        #[cfg(feature = "simd")]
        let mut parsed_language = simd_json::from_reader(reader).map_err(Error::SimdJson)?;

        if deduplicate {
            #[allow(deprecated)]
            self.deduplicate(&mut parsed_language);
        }

        self.languages
            .insert(language.to_owned(), Language::Data(parsed_language));
        Ok(())
    }

    /// Loads a language from a `&[u8]` of Hydrogen I18n's JSON.
    ///
    /// ## Arguments
    ///
    /// If check_link is `true` and the language is a link, it will check if the language exists.
    ///
    /// If deduplicate is `true`, translations and categories equal to the default language will be removed.
    #[deprecated(since = "2.2.0", note = "Use a builder instead")]
    pub fn from_slice(
        &mut self,
        language: &str,
        data: &[u8],
        check_link: bool,
        deduplicate: bool,
    ) -> Result<()> {
        #[allow(deprecated)]
        if let Some(link) = data.strip_prefix("_link:".as_bytes()) {
            let link_str = std::str::from_utf8(link).map_err(Error::Utf8)?;

            if check_link && !self.languages.contains_key(link_str) {
                return Err(Error::LanguageNotFound(link_str.to_owned()));
            }

            self.languages
                .insert(language.to_owned(), Language::Link(link_str.to_owned()));
        } else {
            #[cfg(not(feature = "simd"))]
            let mut parsed_language = serde_json::from_slice(data).map_err(Error::Json)?;

            #[cfg(feature = "simd")]
            let mut parsed_language = {
                let mut data = data.to_owned();
                simd_json::from_slice(&mut data).map_err(Error::SimdJson)
            }?;

            if deduplicate {
                self.deduplicate(&mut parsed_language);
            }

            self.languages
                .insert(language.to_owned(), Language::Data(parsed_language));
        }

        Ok(())
    }

    /// Loads a language from a [serde_json::Value] of Hydrogen I18n's JSON.
    ///
    /// ## Arguments
    ///
    /// If deduplicate is `true`, translations and categories equal to the default language will be removed.
    #[deprecated(since = "2.2.0", note = "Use a builder instead")]
    pub fn from_value(
        &mut self,
        language: &str,
        data: serde_json::Value,
        deduplicate: bool,
    ) -> serde_json::Result<()> {
        let mut parsed_language = serde_json::from_value(data)?;

        if deduplicate {
            #[allow(deprecated)]
            self.deduplicate(&mut parsed_language);
        }

        self.languages
            .insert(language.to_owned(), Language::Data(parsed_language));
        Ok(())
    }

    /// Sets the default language from the languages already loaded.
    ///
    /// ## Arguments
    ///
    /// If deduplicate is `true`, the language will be removed instead of cloned.
    ///
    /// ## Returns
    ///
    /// `true` if the language was found and set as default.
    ///
    /// ## Notes
    ///
    /// This function will fail if the language is a link.
    #[deprecated(since = "2.2.0", note = "Use a builder instead")]
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
    ///
    /// ## Arguments
    ///
    /// If deduplicate is `true`, translations and categories equal to the default language will be removed.
    #[deprecated(since = "2.2.0", note = "Use a builder instead")]
    pub fn from_file<P: AsRef<Path>>(
        &mut self,
        language: &str,
        path: P,
        deduplicate: bool,
    ) -> Result<()> {
        let file = File::open(path).map_err(Error::Io)?;
        let buffered_reader = BufReader::new(file);
        #[allow(deprecated)]
        self.from_reader(language, buffered_reader, deduplicate)
    }

    /// Loads all languages from a directory containing files of Hydrogen I18n's JSON, ignoring files that give errors.
    ///
    /// ## Arguments
    ///
    /// If deduplicate is `true`, translations and categories equal to the default language will be removed.
    ///
    /// ## Notes
    ///
    /// When loading a language, the file name will be used as the language name.
    ///
    /// All files loaded will be parsed as languages, ignoring links.
    #[deprecated(since = "2.2.0", note = "Use a builder instead")]
    pub fn from_dir<P: AsRef<Path>>(&mut self, path: P, deduplicate: bool) -> Result<()> {
        #[allow(deprecated)]
        for entry in (path.as_ref().read_dir().map_err(Error::Io)?).flatten() {
            let path = entry.path();
            if let Some(language) = path
                .file_stem()
                .and_then(|s| s.to_str().map(|f| f.to_owned()))
            {
                _ = self.from_file(&language, path, deduplicate);
            }
        }

        Ok(())
    }

    /// Loads all languages from a directory containing files of Hydrogen I18n's JSON, ignoring files that give errors.
    ///
    /// ## Arguments
    ///
    /// If check_link is `true` when loading a link, it will check if the language exists.
    ///
    /// If deduplicate is `true`, translations and categories equal to the default language will be removed.
    ///
    /// ## Notes
    ///
    /// When loading a language, the file name will be used as the language name.
    ///
    /// This function considers the file extension as your content type, *.json for languages and *.link for links.
    #[deprecated(since = "2.2.0", note = "Use a builder instead")]
    pub fn from_dir_with_links<P: AsRef<Path>>(
        &mut self,
        path: P,
        check_link: bool,
        deduplicate: bool,
    ) -> Result<()> {
        #[allow(deprecated)]
        for entry in (path.as_ref().read_dir().map_err(Error::Io)?).flatten() {
            let path = entry.path();

            match entry.path().extension().and_then(|s| s.to_str()) {
                Some("json") => {
                    if let Some(language) = path
                        .file_stem()
                        .and_then(|s| s.to_str().map(|f| f.to_owned()))
                    {
                        _ = self.from_file(&language, path, deduplicate);
                    }
                }
                Some("link") => {
                    if let Some(language) = path
                        .file_stem()
                        .and_then(|s| s.to_str().map(|f| f.to_owned()))
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

        Ok(())
    }

    /// Gets a language resolving link.
    ///
    /// ## Returns
    ///
    /// `None` if the language doesn't exist, if link isn't valid or if the link points to another link.
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
        self.default.get(category)?.get(key).cloned()
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
            .cloned()
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
    #[deprecated(since = "2.2.0", note = "Use a builder instead")]
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

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
impl I18n {
    /// Loads a language or a link from a file of Hydrogen I18n's JSON using Tokio.
    ///
    /// ## Arguments
    ///
    /// If deduplicate is `true`, translations and categories equal to the default language will be removed.
    ///
    /// ## Notes
    ///
    /// Internally uses [tokio::task::spawn_blocking] to load and parse the file.
    ///
    /// Deduplication can't be done in another thread using Tokio, so it will be executed in the current thread.
    #[deprecated(since = "2.2.0", note = "Use `builders::TokioI18nBuilder` instead")]
    pub async fn tokio_from_file<P: AsRef<Path> + std::marker::Send + 'static>(
        &mut self,
        language: &str,
        path: P,
        deduplicate: bool,
    ) -> Result<()> {
        let mut parsed_language = tokio::task::spawn_blocking(|| {
            let file = File::open(path).map_err(Error::Io)?;
            let buffered_reader = BufReader::new(file);

            #[cfg(not(feature = "simd"))]
            let data = serde_json::from_reader(buffered_reader).map_err(Error::Json);

            #[cfg(feature = "simd")]
            let data = simd_json::from_reader(buffered_reader).map_err(Error::SimdJson);

            data
        })
        .await
        .map_err(Error::Tokio)??;

        if deduplicate {
            #[allow(deprecated)]
            self.deduplicate(&mut parsed_language);
        }

        self.languages
            .insert(language.to_owned(), Language::Data(parsed_language));

        Ok(())
    }

    /// Loads all languages from a directory containing files of Hydrogen I18n's JSON, ignoring files that give errors.
    ///
    /// ## Arguments
    ///
    /// If deduplicate is `true`, translations and categories equal to the default language will be removed.
    ///
    /// ## Notes
    ///
    /// When loading a language, the file name will be used as the language name.
    ///
    /// All files loaded will be parsed as languages, ignoring links.
    ///
    /// Internally uses [tokio::task::spawn_blocking] to load and parse the file.
    ///
    /// Deduplication can't be done in another thread using Tokio, so it will be executed in the current thread.
    #[deprecated(since = "2.2.0", note = "Use `builders::TokioI18nBuilder` instead")]
    pub async fn tokio_from_dir<P: AsRef<Path>>(
        &mut self,
        path: P,
        deduplicate: bool,
    ) -> Result<()> {
        #[allow(deprecated)]
        for entry in (path.as_ref().read_dir().map_err(Error::Io)?).flatten() {
            let path = entry.path();
            if let Some(language) = path
                .file_stem()
                .and_then(|s| s.to_str().map(|f| f.to_owned()))
            {
                _ = self.tokio_from_file(&language, path, deduplicate).await;
            }
        }

        Ok(())
    }

    /// Loads all languages from a directory containing files of Hydrogen I18n's JSON, ignoring files that give errors.
    ///
    /// ## Arguments
    ///
    /// If check_link is `true` when loading a link, it will check if the language exists.
    ///
    /// If deduplicate is `true`, translations and categories equal to the default language will be removed.
    ///
    /// ## Notes
    ///
    /// When loading a language, the file name will be used as the language name.
    ///
    /// This function considers the file extension as your content type, *.json for languages and *.link for links.
    ///
    /// Internally uses [tokio::task::spawn_blocking] to load and parse the file.
    ///
    /// Deduplication can't be done in another thread using Tokio, so it will be executed in the current thread.
    #[deprecated(since = "2.2.0", note = "Use `builders::TokioI18nBuilder` instead")]
    pub async fn tokio_from_dir_with_links<P: AsRef<Path>>(
        &mut self,
        path: P,
        check_link: bool,
        deduplicate: bool,
    ) -> Result<()> {
        #[allow(deprecated)]
        for entry in (path.as_ref().read_dir().map_err(Error::Io)?).flatten() {
            let path = entry.path();

            match entry.path().extension().and_then(|s| s.to_str()) {
                Some("json") => {
                    if let Some(language) = path
                        .file_stem()
                        .and_then(|s| s.to_str().map(|f| f.to_owned()))
                    {
                        _ = self.tokio_from_file(&language, path, deduplicate).await;
                    }
                }
                Some("link") => {
                    if let Some(language) = path
                        .file_stem()
                        .and_then(|s| s.to_str().map(|f| f.to_owned()))
                    {
                        let file = tokio::fs::File::open(path).await.map_err(Error::Io)?;
                        let mut data = String::new();
                        let Ok(_) = file.take(16).read_to_string(&mut data).await else {
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

        Ok(())
    }
}

#[cfg(feature = "serenity")]
#[cfg_attr(docsrs, doc(cfg(feature = "serenity")))]
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
