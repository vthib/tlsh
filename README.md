# TLSH2

[![Build status](https://github.com/vthib/tlsh/actions/workflows/ci.yml/badge.svg)](https://github.com/vthib/boreal/tlsh/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/tlsh2.svg)](https://crates.io/crates/tlsh2)
[![Documentation](https://docs.rs/tlsh2/badge.svg)](https://docs.rs/tlsh2)

Rust port of the [TLSH library](https://github.com/trendmicro/tlsh).
The code is kept close to the original C++ version, to limit bugs and help maintainability

This crate is `no_std` and different configurations of bucket numbers and checksum length
are handled as generics, making every configuration properly optimized.

```rust
// The default builder uses 128 buckets and a 1-byte checksum.
// Other builders are also available.
let mut builder = tlsh2::TlshDefaultBuilder::new();
builder.update(b"Sed ut perspiciatis unde omnis iste natus");
builder.update(b"error sit voluptatem accusantium");
let tlsh = builder.build()
    .ok_or_else(|| "could not generate TLSH from payload")?;

// Alternatively, a TLSH object can be generated directly from
// a byte slice.
let tlsh2 = tlsh2::TlshDefaultBuilder::build_from(
    b"odit aut fugit, sed quia consequuntur magni dolores"
).ok_or_else(|| "could not generate TLSH from second payload")?;

// Then, the TLSH object can be used to generated a hash or compute
// distances
assert_eq!(
    tlsh.hash(),
    b"T184A022B383C2A2A20ACB0830880CF0202CCAC080033A023800338\
      A30B0880AA8E0BE38".as_slice(),
);
// The `diff` feature is required for this computation.
assert_eq!(tlsh.diff(&tlsh2, true), 209);
```

Those configurations are available:
- 128 buckets and 1-byte checksum (default).
- 128 buckets and 3-byte checksum.
- 256 buckets and 1-byte checksum.
- 256 buckets and 3-byte checksum.
- 48 buckets and 1-byte checksum.

The `fast` feature speeds up TLSH generation but adds a 64kB lookup table.

The `threaded` and `private` options that exists in the original TLSH version
are not yet implemented.