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

use std::collections::HashMap;

/// Re-export of the `serde_json` crate.
pub use serde_json;

/// A group of translations.
pub type Category = HashMap<String, String>;
/// Translations for a language.
pub type Language = HashMap<String, Category>;

/// Translation manager, used to load and manage all the languages supported.
#[derive(Clone, Default)]
pub struct I18n {
    /// All languages loaded and available in this manager.
    languages: HashMap<String, Language>,
    /// The default language.
    default: Language,
}

impl I18n {
    /// Creates a new instance of the manager, proving the default language and the languages already loaded.
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

    /// Creates a new instance of the manager, proving the languages already loaded.
    pub fn new_with_languages(languages: HashMap<String, Language>) -> Self {
        Self::new_with_default_and_languages(HashMap::new(), languages)
    }

    /// Creates a new instance of the manager.
    pub fn new() -> Self {
        Self::new_with_default_and_languages(HashMap::new(), HashMap::new())
    }
}
