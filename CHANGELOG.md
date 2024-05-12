# Hydrogen I18n // Changelog

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

### Fixes

- `Result` and `Error` are only used with `std` feature, but are available without them.

## [1.0.0] - 2023-07-28

### Added

- File and directory loading functions.
- Internal types that are used by the `Translator` to store translations.
- `Translator` struct for managing translations.
- Support for environments without the `std`.
- Utility functions for Serenity.
