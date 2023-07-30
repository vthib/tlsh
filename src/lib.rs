//! TLSH
//!
//! A Rust implementation of the TLSH algorithm.
//! The code is kept close to the original C++ version, to limit bugs and help
//! maintainability.
//!
//! This crate is `no_std` and different configurations of bucket numbers and
//! checksum length are handled as generics, making every configuration properly
//! optimized.
//!
//! ```
//! // The default builder uses 128 buckets and a 1-byte checksum.
//! // Other builders are also available.
//! let mut builder = tlsh2::TlshDefaultBuilder::new();
//! builder.update(b"Sed ut perspiciatis unde omnis iste natus");
//! builder.update(b"error sit voluptatem accusantium");
//! let tlsh = builder.build()
//!     .ok_or_else(|| "could not generate TLSH from payload")?;
//!
//! // Alternatively, a TLSH object can be generated directly from
//! // a byte slice.
//! let tlsh2 = tlsh2::TlshDefaultBuilder::build_from(
//!     b"odit aut fugit, sed quia consequuntur magni dolores"
//! ).ok_or_else(|| "could not generate TLSH from second payload")?;
//!
//! // Then, the TLSH object can be used to generated a hash or compute
//! // distances
//! assert_eq!(
//!     tlsh.hash(),
//!     b"T184A022B383C2A2A20ACB0830880CF0202CCAC080033A023800338\
//!       A30B0880AA8E0BE38".as_slice(),
//! );
//! // The `diff` feature is required for this computation.
//! #[cfg(feature = "diff")]
//! assert_eq!(tlsh.diff(&tlsh2, true), 209);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
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

/// Builder with 256 buckets and a 1 byte checksum.
pub type TlshBuilder256_1 = TlshBuilder<256, 1, 64, 136, 50>;
/// TLSH with 256 buckets and a 1 byte checksum.
pub type Tlsh256_1 = Tlsh<1, 136, 64>;

/// Builder with 128 buckets and a 1 byte checksum.
pub type TlshBuilder128_1 = TlshBuilder<128, 1, 32, 72, 50>;
/// TLSH with 128 buckets and a 1 byte checksum.
pub type Tlsh128_1 = Tlsh<1, 72, 32>;

/// Builder with 48 buckets and a 1 byte checksum.
pub type TlshBuilder48_1 = TlshBuilder<48, 1, 12, 32, 10>;
/// TLSh with 48 buckets and a 1 byte checksum.
pub type Tlsh48_1 = Tlsh<1, 32, 12>;

/// Builder with 256 buckets and a 3 bytes checksum.
pub type TlshBuilder256_3 = TlshBuilder<256, 3, 64, 140, 50>;
/// TLSH with 256 buckets and a 3 bytes checksum.
pub type Tlsh256_3 = Tlsh<3, 140, 64>;

/// Builder with 128 buckets and a 3 bytes checksum.
pub type TlshBuilder128_3 = TlshBuilder<128, 3, 32, 76, 50>;
/// TLSH with 128 buckets and a 3 bytes checksum.
pub type Tlsh128_3 = Tlsh<3, 76, 32>;

/// Default builder, using 128 buckets and a 1 byte checksum.
pub type TlshDefaultBuilder = TlshBuilder128_1;
/// Default TLSH, using 128 buckets and a 1 byte checksum.
pub type TlshDefault = Tlsh128_1;
