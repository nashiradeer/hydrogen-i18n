# Hydrogen: I18n // Changelog

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
