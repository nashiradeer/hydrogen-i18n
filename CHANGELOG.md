# Hydrogen I18n // Changelog

## [Unreleased]

### Added

- Create loaders used internally.
- Create file loader.
- Create directory loader.

### Changed

- Update 'serenity' to 0.12.0.
- Rename 'HydrogenI18n' to 'I18n', refactoring it.
- Set 'doc_cfg' feature when in docsrs.
- Set 'missing_docs' attribute as warn.

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
