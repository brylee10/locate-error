# Changelog
This documents the main changes to the `locate-error` and associated crates.

## [0.1.1] - 2025-04-04
### Added
Initial version of `locate-error`

Supports
- The `Location` type which holds a file, column, and line number 
- The `Locate` derive macro which uses the `#[locate_from]` attribute to implement the `From<Inner> for Outer` trait for the modified inner error type.
- The `location` macro which returns a `Location` corresponding to the call site

### Fixed
- Version `0.1.0` (yanked) had an issue with `Location` field visibility which is adjusted in this version
