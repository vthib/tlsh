# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added 

- Added TLSH difference computation (with the `diff` feature).
- The crate is now `no_std`.
- Tests now use reference files from the original TLSH repo to check conformance.
- `TlshCore` renamed to `TlshBuilder`, new `Tlsh` object to compute hashes and differences.

### Fixed

- Fixed tests when run on Windows.

### Removed

- Remove `showvers` parameter in hash function. Hashes now always return the `T1` prefix.

## [0.1.0] - 2022-12-30

Initial release