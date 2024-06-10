//! Async [`I18n`] builder using [tokio](https://docs.rs/tokio/).

//! Synchronous builder for [`I18n`] struct.

use async_recursion::async_recursion;
use std::{collections::HashMap, path::Path, str::from_utf8};
use tokio::{
    fs::File,
    io::{AsyncRead, AsyncReadExt, BufReader},
    sync::RwLock,
    task::spawn_blocking,
};

use crate::{
    parsers::{parse_from_slice, parse_from_str},
    resolve_translation, Category, Error, I18n, Language, Result,
};

/// [`I18n`] builder.
pub struct TokioI18nBuilder {
    /// Languages data.
    languages: RwLock<HashMap<String, HashMap<String, Category>>>,
    /// Languages links.
    links: RwLock<HashMap<String, String>>,
    /// Default language.
    default_language: RwLock<String>,
}

impl TokioI18nBuilder {
    /// Creates a new [`TokioI18nBuilder`].
    pub fn new(default_language: &str) -> Self {
        Self {
            languages: RwLock::new(HashMap::new()),
            links: RwLock::new(HashMap::new()),
            default_language: RwLock::new(default_language.to_owned()),
        }
    }

    /// Adds a language to the builder.
    pub async fn add_language(&self, language: &str, categories: HashMap<String, Category>) {
        self.languages
            .write()
            .await
            .insert(language.to_owned(), categories);
    }

    /// Adds a link to the builder.
    pub async fn add_link(&self, language: &str, link: &str) {
        self.links
            .write()
            .await
            .insert(language.to_owned(), link.to_owned());
    }

    /// Sets the default language.
    pub async fn set_default_language(&self, language: &str) {
        self.default_language
            .write()
            .await
            .clone_from(&language.to_owned());
    }

    /// Gets the default language.
    pub async fn get_default_language(&self) -> String {
        self.default_language.read().await.clone()
    }

    /// Adds a language from a string.
    pub async fn add_from_str(&self, language: &str, mut string: String) -> Result<()> {
        if let Some(link_content) = string.strip_prefix("_link:") {
            self.add_link(language, link_content).await;
        } else {
            let json = spawn_blocking(move || parse_from_str(&mut string))
                .await
                .map_err(Error::Tokio)??;

            self.add_language(language, json).await;
        }

        Ok(())
    }

    /// Adds a language from a slice.
    pub async fn add_from_slice(&self, language: &str, mut slice: Vec<u8>) -> Result<()> {
        let mut temp_buffer = [0; 6];

        if slice.len() < 6 {
            let json = spawn_blocking(move || parse_from_slice(&mut slice))
                .await
                .map_err(Error::Tokio)??;

            self.add_language(language, json).await;

            return Ok(());
        }

        temp_buffer.copy_from_slice(&slice[..6]);

        let temp_buffer_str = from_utf8(&temp_buffer).map_err(Error::Utf8)?;

        if temp_buffer_str == "_link:" {
            let link_content = from_utf8(&slice[6..]).map_err(Error::Utf8)?;

            self.add_link(language, link_content).await;
        } else {
            let json = spawn_blocking(move || parse_from_slice(&mut slice))
                .await
                .map_err(Error::Tokio)??;

            self.add_language(language, json).await;
        }

        Ok(())
    }

    /// Adds a language from a reader.
    ///
    /// Different from the synchronous version, this method stores the reader's content in a buffer before parsing it.
    pub async fn add_from_reader<R: AsyncRead + Unpin>(
        &self,
        language: &str,
        mut reader: R,
    ) -> Result<()> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).await.map_err(Error::Io)?;
        self.add_from_slice(language, buffer).await
    }

    /// Adds a language from a file.
    pub async fn add_from_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();

        let language_name = path
            .file_stem()
            .ok_or(Error::InvalidFileName)?
            .to_str()
            .ok_or(Error::InvalidFileName)?;

        let file = File::open(path).await.map_err(Error::Io)?;
        let reader = BufReader::new(file);

        self.add_from_reader(language_name, reader).await
    }

    /// Adds languages from a directory.
    ///
    /// This will add all files in the directory and its subdirectories.
    #[async_recursion]
    pub async fn add_from_dir<P: AsRef<Path> + Send>(&self, path: P) -> Result<()> {
        let path = path.as_ref();

        for entry in (path.read_dir().map_err(Error::Io)?).flatten() {
            let path = entry.path();

            if path.is_file() {
                self.add_from_file(path).await?;
            } else if path.is_dir() {
                self.add_from_dir(path).await?;
            }
        }

        Ok(())
    }

    /// Removes a language from the builder.
    pub async fn remove_language(&self, language: &str) {
        self.languages.write().await.remove(language);
    }

    /// Removes a link from the builder.
    pub async fn remove_link(&self, language: &str) {
        self.links.write().await.remove(language);
    }

    /// Removes all languages from the builder.
    pub async fn clear_languages(&self) {
        self.languages.write().await.clear();
    }

    /// Removes all links from the builder.
    pub async fn clear_links(&self) {
        self.links.write().await.clear();
    }

    /// Gets the languages.
    ///
    /// Different from the synchronous version, this method returns a clone of the languages to avoid returning the lock guard.
    pub async fn get_languages(&self) -> HashMap<String, HashMap<String, Category>> {
        self.languages.read().await.clone()
    }

    /// Gets the links.
    pub async fn get_links(&self) -> HashMap<String, String> {
        self.links.read().await.clone()
    }

    /// Builds the [`I18n`] struct.
    pub async fn build(self) -> Result<I18n> {
        let mut self_languages = self.languages.into_inner();
        let self_links = self.links.into_inner();
        let self_default_language = self.default_language.into_inner();

        let Some(default_language) = self_languages.remove(&self_default_language) else {
            return Err(Error::LanguageNotFound(self_default_language.clone()));
        };

        let mut languages = HashMap::new();

        for (language, mut categories) in self_languages {
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

        for (language, link) in self_links {
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

impl Default for TokioI18nBuilder {
    fn default() -> Self {
        Self::new("")
    }
}
