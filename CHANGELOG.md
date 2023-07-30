# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1] - 2023-05-04

### Fixed

- Fix overflow panic in debug builds when total length of data exceeds 1408534 bytes
  (~1.34MB). See [fcdf710](https://github.com/vthib/tlsh/commit/fcdf710ac5730e68f4e4bf987623bc0e9b8e0819).

## [0.2.0] - 2023-01-01

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
