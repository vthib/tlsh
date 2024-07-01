# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2024-07-02

### Added

- Added `fast` feature to improve speed of hashing algorithm, to the cost of an additional
  64k table. See [#8](https://github.com/vthib/tlsh/pull/8).

### Changed

- Improved hashing algorithm performance. See [#8](https://github.com/vthib/tlsh/pull/8).

## [0.3.0] - 2023-07-30

### Added

- Added FromStr implementation for `Tlsh`, to be able to build a `Tlsh` from a hash string.
  See #5.
- Added type aliases for the `Tlsh` object, mirroring the aliases already existing on
  the `TlshBuilder` object.
  See [f242ca9](https://github.com/vthib/tlsh/commit/f242ca963f46ac59c96b0b3af23f0263ed7d18c4).

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
