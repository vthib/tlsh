//! TLSH hash
//!
//! A Rust implementation of the TLSH algorithm.
//! The code is kept close to the original C++ version, to limit bugs and help maintainability.
#![no_std]
#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::cargo)]

mod pearson;
mod quartile;
mod tlsh;
mod util;

const BUCKETS: usize = 256;

pub use crate::tlsh::{Tlsh, TlshBuilder};

/// Hasher with 256 buckets and a 1 byte checksum.
pub type TlshBuilder256_1 = TlshBuilder<256, 1, 64, 136, 50>;
/// Hasher with 128 buckets and a 1 byte checksum.
pub type TlshBuilder128_1 = TlshBuilder<128, 1, 32, 72, 50>;
/// Hasher with 48 buckets and a 1 byte checksum.
pub type TlshBuilder48_1 = TlshBuilder<48, 1, 12, 32, 10>;

/// Hasher with 256 buckets and a 3 bytes checksum.
pub type TlshBuilder256_3 = TlshBuilder<256, 3, 64, 140, 50>;
/// Hasher with 128 buckets and a 3 bytes checksum.
pub type TlshBuilder128_3 = TlshBuilder<128, 3, 32, 76, 50>;

/// Default hasher, using 128 buckets and a 1 byte checksum.
pub type TlshDefaultBuilder = TlshBuilder128_1;
