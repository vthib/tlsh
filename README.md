# TLSH2

[![Build status](https://github.com/vthib/tlsh/actions/workflows/ci.yml/badge.svg)](https://github.com/vthib/boreal/tlsh/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/tlsh2.svg)](https://crates.io/crates/tlsh2)
[![Documentation](https://docs.rs/tlsh2/badge.svg)](https://docs.rs/tlsh2)

Rust port of the [TLSH hash function](https://github.com/trendmicro/tlsh).
The code is kept close to the original C++ version, to limit bugs and help maintainability

For the moment, only the default hashes are implemented, with different bucket sizes (48, 128, 256)
and different checksum sizes (1 byte, 3 bytes).
Hashing options (private, threaded) as well as distance computation are not yet available, but are
easy to add should they be needed.
