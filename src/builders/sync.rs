//! Synchronous builder for [`I18n`] struct.

use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    path::Path,
    str::from_utf8,
};

use crate::{
    parsers::{parse_from_reader, parse_from_slice, parse_from_str},
    resolve_translation, Category, Error, I18n, Language, Result,
};

/// [`I18n`] builder.
pub struct I18nBuilder {
    /// Languages data.
    languages: HashMap<String, HashMap<String, Category>>,
    /// Languages links.
    links: HashMap<String, String>,
    /// Default language.
    default_language: String,
}

impl I18nBuilder {
    /// Creates a new [`I18nBuilder`].
    pub fn new(default_language: &str) -> Self {
        Self {
            languages: HashMap::new(),
            links: HashMap::new(),
            default_language: default_language.to_owned(),
        }
    }

    /// Adds a language to the builder.
    pub fn add_language(mut self, language: &str, categories: HashMap<String, Category>) -> Self {
        self.languages.insert(language.to_owned(), categories);
        self
    }

    /// Adds a link to the builder.
    pub fn add_link(mut self, language: &str, link: &str) -> Self {
        self.links.insert(language.to_owned(), link.to_owned());
        self
    }

    /// Sets the default language.
    pub fn set_default_language(mut self, language: &str) -> Self {
        language.clone_into(&mut self.default_language);
        self
    }

    /// Gets the default language.
    pub fn get_default_language(&self) -> &str {
        &self.default_language
    }

    /// Adds a language from a string.
    pub fn add_from_str(self, language: &str, string: &str) -> Result<Self> {
        if let Some(link_content) = string.strip_prefix("_link:") {
            Ok(self.add_link(language, link_content))
        } else {
            Ok(self.add_language(language, parse_from_str(&mut language.to_owned())?))
        }
    }

    /// Adds a language from a slice.
    pub fn add_from_slice(self, language: &str, slice: &mut [u8]) -> Result<Self> {
        let mut temp_buffer = [0; 6];

        if slice.len() < 6 {
            return Ok(self.add_language(language, parse_from_slice(slice)?));
        }

        temp_buffer.copy_from_slice(&slice[..6]);

        let temp_buffer_str = from_utf8(&temp_buffer).map_err(Error::Utf8)?;

        if temp_buffer_str == "_link:" {
            let link_content = from_utf8(&slice[6..]).map_err(Error::Utf8)?;

            Ok(self.add_link(language, link_content))
        } else {
            Ok(self.add_language(language, parse_from_slice(slice)?))
        }
    }

    /// Adds a language from a reader.
    pub fn add_from_reader<R: Read + Seek>(self, language: &str, mut reader: R) -> Result<Self> {
        let mut temp_buffer = [0; 6];

        if let Err(e) = reader.read_exact(&mut temp_buffer) {
            if e.kind() == std::io::ErrorKind::UnexpectedEof {
                reader.seek(SeekFrom::Start(0)).map_err(Error::Io)?;

                return Ok(self.add_language(language, parse_from_reader(reader)?));
            } else {
                return Err(Error::Io(e));
            }
        }

        let temp_buffer_str = from_utf8(&temp_buffer).map_err(Error::Utf8)?;

        if temp_buffer_str == "_link:" {
            let mut link_content = String::new();
            reader
                .read_to_string(&mut link_content)
                .map_err(Error::Io)?;

            Ok(self.add_link(language, &link_content))
        } else {
            reader.seek(SeekFrom::Start(0)).map_err(Error::Io)?;

            Ok(self.add_language(language, parse_from_reader(reader)?))
        }
    }

    /// Adds a language from a file.
    pub fn add_from_file<P: AsRef<Path>>(self, path: P) -> Result<Self> {
        let path = path.as_ref();

        let language_name = path
            .file_stem()
            .ok_or(Error::InvalidFileName)?
            .to_str()
            .ok_or(Error::InvalidFileName)?;

        let file = File::open(path).map_err(Error::Io)?;
        let reader = BufReader::new(file);

        self.add_from_reader(language_name, reader)
    }

    /// Adds languages from a directory.
    ///
    /// This will add all files in the directory and its subdirectories.
    pub fn add_from_dir<P: AsRef<Path>>(mut self, path: P) -> Result<Self> {
        let path = path.as_ref();

        for entry in (path.read_dir().map_err(Error::Io)?).flatten() {
            let path = entry.path();

            if path.is_file() {
                self = self.add_from_file(path)?;
            } else if path.is_dir() {
                self = self.add_from_dir(path)?;
            }
        }

        Ok(self)
    }

    /// Removes a language from the builder.
    pub fn remove_language(mut self, language: &str) -> Self {
        self.languages.remove(language);
        self
    }

    /// Removes a link from the builder.
    pub fn remove_link(mut self, language: &str) -> Self {
        self.links.remove(language);
        self
    }

    /// Removes all languages from the builder.
    pub fn clear_languages(mut self) -> Self {
        self.languages.clear();
        self
    }

    /// Removes all links from the builder.
    pub fn clear_links(mut self) -> Self {
        self.links.clear();
        self
    }

    /// Gets the languages.
    pub fn get_languages(&self) -> &HashMap<String, HashMap<String, Category>> {
        &self.languages
    }

    /// Gets the languages mutable.
    pub fn get_language_mut(&mut self) -> &mut HashMap<String, HashMap<String, Category>> {
        &mut self.languages
    }

    /// Gets the links.
    pub fn get_links(&self) -> &HashMap<String, String> {
        &self.links
    }

    /// Gets the links mutable.
    pub fn get_links_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.links
    }

    /// Builds the [`I18n`] struct.
    pub fn build(mut self) -> Result<I18n> {
        let Some(default_language) = self.languages.remove(&self.default_language) else {
            return Err(Error::LanguageNotFound(self.default_language));
        };

        let mut languages = HashMap::new();

        for (language, mut categories) in self.languages {
            categories.retain(|category_name, category| {
                category.retain(|key, value| {
                    if let Some(default_translation) =
                        resolve_translation(&default_language, category_name, key)
                    {
                        if *default_translation == *value {
                            return false;
                        }
                    }

                    true
                });

                !category.is_empty()
            });

            languages.insert(language, Language::Data(categories));
        }

        for (language, link) in self.links {
            if !languages.contains_key(&language) && languages.contains_key(&link) {
                languages.insert(language, Language::Link(link));
            }
        }

        Ok(I18n {
            languages,
            default: default_language,
        })
    }
}

impl Default for I18nBuilder {
    fn default() -> Self {
        Self::new("")
    }
}
