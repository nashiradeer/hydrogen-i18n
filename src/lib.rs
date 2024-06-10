#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # Nashira Deer // Hydrogen I18n
//!
//! Translation utilities for server-side applications that need to deal with different languages at the same time.
//!
//! [![PayPal](https://img.shields.io/badge/Paypal-003087?style=for-the-badge&logo=paypal&logoColor=%23fff)](https://www.paypal.com/donate/?business=QQGMTC3FQAJF6&no_recurring=0&item_name=Thanks+for+donating+for+me%2C+this+helps+me+a+lot+to+continue+developing+and+maintaining+my+projects.&currency_code=USD)
//! [![GitHub Sponsor](https://img.shields.io/badge/GitHub%20Sponsor-181717?style=for-the-badge&logo=github&logoColor=%23fff)](https://github.com/sponsors/nashiradeer)
//! [![Crates.io](https://img.shields.io/crates/v/hydrogen-i18n?style=for-the-badge&logo=rust&logoColor=%23fff&label=Crates.io&labelColor=%23000&color=%23000)](https://crates.io/crates/hydrogen-i18n)
//! [![docs.rs](https://img.shields.io/docsrs/hydrogen-i18n?style=for-the-badge&logo=docsdotrs&logoColor=%23fff&label=Docs.rs&labelColor=%23000&color=%23000)](https://docs.rs/hydrogen-i18n/)
//!
//! Server-side applications that deal directly with users will need to be prepared to deal with different languages too, so Hydrogen I18n comes with utilities focused on making this task easier, loading and managing all the languages supported by the application in the memory, avoiding unnecessary disk and CPU usage.
//!
//! Hydrogen I18n is part of [Hydrogen Framework](https://github.com/users/nashiradeer/projects/8), this means that this crate is designed to be used on Discord bots created using [serenity](https://crates.io/crates/serenity) and [twilight](https://crates.io/crates/twilight), but is sufficiently generic to be used by any other application that does not use these Discord libraries or even Discord bots.
//!
//! ## Donating
//!
//! Consider donating to make Hydrogen I18n development possible. You can donate through Nashira Deer's [PayPal](https://www.paypal.com/donate/?business=QQGMTC3FQAJF6&no_recurring=0&item_name=Thanks+for+donating+for+me%2C+this+helps+me+a+lot+to+continue+developing+and+maintaining+my+projects.&currency_code=USD) or [GitHub Sponsor](https://github.com/sponsors/nashiradeer).
//!
//! ## Features
//!
//! - `serenity`: Enables functions that make it easy to use the library in Discord apps and bots built with [serenity](https://crates.io/crates/serenity).
//! - `tokio`: Enables [tokio](https://crates.io/crates/tokio)-based builder.
//! - `simd`: Enables [simd-json](https://crates.io/crates/simd-json)-based parser and use it by default.
//!
//! ## Credits
//!
//! Hydrogen I18n is a Nashira Deer project licensed under the [MIT License](https://github.com/nashiradeer/hydrogen-i18n/blob/main/LICENSE.txt) and licensed under the [GNU Lesser General Public License v3](https://github.com/nashiradeer/hydrogen-i18n/blob/c00b016356dc9263571e6cc6ede87969bf31bf02/LICENSE.txt) until v1.0.1.

use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    io, result,
};

pub mod builders;
pub mod parsers;

mod i18n;
pub use i18n::*;

/// Groups all the errors that can be returned by Hydrogen I18n.
#[derive(Debug)]
pub enum Error {
    /// An error related to IO.
    Io(io::Error),

    /// An error related to JSON parsing.
    Json(serde_json::Error),

    /// The language was not found.
    LanguageNotFound(String),

    /// An invalid file name.
    InvalidFileName,

    /// An error related to UTF-8 parsing.
    Utf8(std::str::Utf8Error),

    #[cfg(feature = "tokio")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
    /// An error related to Tokio.
    Tokio(tokio::task::JoinError),

    #[cfg(feature = "simd")]
    #[cfg_attr(docsrs, doc(cfg(feature = "simd")))]
    /// An error related to SIMD JSON parsing.
    SimdJson(simd_json::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "IO error: {}", error),
            Self::Json(error) => write!(f, "JSON error: {}", error),
            Self::InvalidFileName => write!(f, "Invalid file name"),
            Self::LanguageNotFound(language) => {
                write!(f, "Language {} not found", language)
            }
            Self::Utf8(error) => write!(f, "UTF-8 error: {}", error),

            #[cfg(feature = "tokio")]
            Self::Tokio(error) => write!(f, "Tokio error: {}", error),

            #[cfg(feature = "simd")]
            Self::SimdJson(error) => write!(f, "SIMD JSON error: {}", error),
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

/// Removes all the translations that are equal to the base language.
#[deprecated(
    since = "2.2.0",
    note = "There's no need to deduplicate languages anymore, use a builder instead"
)]
fn deduplicate_language(
    base: &HashMap<String, Category>,
    deduplicating: &mut HashMap<String, Category>,
) {
    #[allow(deprecated)]
    deduplicating.retain(|category_name, category| {
        category.retain(|key, value| {
            if let Some(default_translation) = resolve_translation(base, category_name, key) {
                if *default_translation == *value {
                    return false;
                }
            }

            true
        });

        !category.is_empty()
    });
}

/// Resolves category and key to a translation without getting the ownership.
fn resolve_translation<'a>(
    language: &'a HashMap<String, Category>,
    category: &str,
    key: &str,
) -> Option<&'a String> {
    language.get(category)?.get(key)
}
