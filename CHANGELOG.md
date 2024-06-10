# Hydrogen I18n // Changelog

## [2.2.0] - 2024-06-10

### Added

- Create `I18nBuilder` struct.
- Create `TokioI18nBuilder` struct.
- Create `Error::InvalidFileName` variant.
- Create `parsers` module to wrap `serde_json` and `simd_json`.

### Changed

- Deprecate `I18n::from_*`, use `I18nBuilder` instead.
- Deprecate `I18n::new*`, use `I18nBuilder` instead.
- Deprecate `I18n::set_default`, use `I18nBuilder` instead.
- Deprecate `I18n::cleanup_links`, use `I18nBuilder` instead.
- Deprecate `I18n` Tokio-based functions, use `TokioI18nBuilder` instead.
- Deprecate private `deduplicate_language`, not necessary anymore.

## [2.1.0] - 2024-05-11

### Added

- Create a function to deduplicate all languages. ([#11](https://github.com/nashiradeer/hydrogen-i18n/issues/11))

### Fixed

- Document the `simd` feature. ([#12](https://github.com/nashiradeer/hydrogen-i18n/issues/12))

## [2.0.0] - 2024-01-17

### Added

- Add support `simd-json` crate.
- Add support `tokio`-based async functions.
- Create a way to link languages.
- Create the deduplicate function.
- Create internal parsers.
- Create 'translation_option' function.

### Changed

- Refactor file loader.
- Refactor directory loader.
- Refactor getters (translate and translate_default).
- Refactor 'serenity' support.
- Rename 'HydrogenI18n' to 'I18n', refactoring it.
- Set the 'doc_cfg' feature when in docsrs.
- Set the 'missing_docs' attribute as warn.
- Update 'serenity' to 0.12.0.

### Removed

- Support for non-std environments.

## [1.0.1] - 2023-07-28

### Fixed

- `Result` and `Error` are only used with `std` feature, but are available without them.

## [1.0.0] - 2023-07-28

### Added

- File and directory loading functions.
- Internal types that are used by the `Translator` to store translations.
- `Translator` struct for managing translations.
- Support for environments without the `std`.
- Utility functions for Serenity.
