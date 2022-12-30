//! TLSH hash
//!
//! A Rust implementation of the TLSH algorithm.
//! The code is kept close to the original C++ version, to limit bugs and help maintainability.

#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::cargo)]

mod pearson;
mod quartile;
mod tlsh;
mod util;

pub use crate::tlsh::TlshCore;

/// Hasher with 256 buckets and a 1 byte checksum.
pub type Tlsh256_1 = crate::tlsh::TlshCore<256, 1, 64, 136, 50>;
/// Hasher with 128 buckets and a 1 byte checksum.
pub type Tlsh128_1 = crate::tlsh::TlshCore<128, 1, 32, 72, 50>;
/// Hasher with 48 buckets and a 1 byte checksum.
pub type Tlsh48_1 = crate::tlsh::TlshCore<48, 1, 12, 30, 10>;

/// Hasher with 256 buckets and a 3 bytes checksum.
pub type Tlsh256_3 = crate::tlsh::TlshCore<256, 3, 64, 136, 50>;
/// Hasher with 128 buckets and a 3 bytes checksum.
pub type Tlsh128_3 = crate::tlsh::TlshCore<128, 3, 32, 72, 50>;

/// Default hasher, using 128 buckets and a 1 byte checksum.
pub type Tlsh = Tlsh128_1;
