# Hydrogen I18n // Changelog

## [Unreleased]

### Added

- Create a way to link language.
- Create deduplicate function.
- Create internal parsers.
- Create 'translation_option' function.

### Changed

- Refactor file loader.
- Refactor directory loader.
- Refactor getters (translate and translate_default).
- Refactor 'serenity' support.
- Rename 'HydrogenI18n' to 'I18n', refactoring it.
- Set 'doc_cfg' feature when in docsrs.
- Set 'missing_docs' attribute as warn.
- Update 'serenity' to 0.12.0.

### Removed

- Support for non-std environments.

## [1.0.1] - 2023-07-28

### Fixes

- `Result` and `Error` are only used with `std` feature, but are available without them.

## [1.0.0] - 2023-07-28

### Added

- File and directory loading functions.
- Internal types used by `Translator` to store translations.
- `Translator` struct for managing translations.
- Support for environments without the `std`.
- Utility functions for Serenity.
