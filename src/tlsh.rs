use core::str::FromStr;

use crate::pearson::{b_mapping, fast_b_mapping};
use crate::quartile::get_quartiles;
use crate::util::{l_capturing, swap_byte};
use crate::BUCKETS;

const SLIDING_WND_SIZE: usize = 5;

const RNG_SIZE: usize = SLIDING_WND_SIZE - 1;

/// Builder object, processing streams of bytes to generate [`Tlsh`] objects.
///
/// You should never provide your own values for the generics, but instead use the pre-configured
/// types such as [`crate::TlshBuilder256_1`] or [`crate::TlshBuilder128_3`].
pub struct TlshBuilder<
    const EFF_BUCKETS: usize,
    const TLSH_CHECKSUM_LEN: usize,
    const CODE_SIZE: usize,
    const TLSH_STRING_LEN_REQ: usize,
    const MIN_DATA_LENGTH: usize,
> {
    a_bucket: [u32; BUCKETS],
    slide_window: [u8; RNG_SIZE],
    checksum: [u8; TLSH_CHECKSUM_LEN],
    data_len: usize,
}

impl<
        const EFF_BUCKETS: usize,
        const TLSH_CHECKSUM_LEN: usize,
        const CODE_SIZE: usize,
        const TLSH_STRING_LEN_REQ: usize,
        const MIN_DATA_LENGTH: usize,
    > Default
    for TlshBuilder<EFF_BUCKETS, TLSH_CHECKSUM_LEN, CODE_SIZE, TLSH_STRING_LEN_REQ, MIN_DATA_LENGTH>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<
        const EFF_BUCKETS: usize,
        const TLSH_CHECKSUM_LEN: usize,
        const CODE_SIZE: usize,
        const TLSH_STRING_LEN_REQ: usize,
        const MIN_DATA_LENGTH: usize,
    > TlshBuilder<EFF_BUCKETS, TLSH_CHECKSUM_LEN, CODE_SIZE, TLSH_STRING_LEN_REQ, MIN_DATA_LENGTH>
{
    /// Create a new TLSH builder.
    pub fn new() -> Self {
        Self {
            a_bucket: [0; BUCKETS],
            slide_window: [0; RNG_SIZE],
            checksum: [0; TLSH_CHECKSUM_LEN],
            data_len: 0,
        }
    }

    /// Generate a [`Tlsh`] object from a given byte slice.
    ///
    /// This is a shorthand for building a [`Tlsh`] object from a single
    /// byte slice, it is equivalent to:
    ///
    /// ```
    /// let data = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit";
    /// let tlsh = tlsh2::TlshDefaultBuilder::build_from(data);
    /// // equivalent to
    /// let mut builder = tlsh2::TlshDefaultBuilder::new();
    /// builder.update(data);
    /// let tlsh = builder.build();
    /// ```
    pub fn build_from(
        data: &[u8],
    ) -> Option<Tlsh<TLSH_CHECKSUM_LEN, TLSH_STRING_LEN_REQ, CODE_SIZE>> {
        let mut builder = Self::new();
        builder.update(data);
        builder.build()
    }

    /// Add bytes into the builder.
    pub fn update(&mut self, data: &[u8]) {
        // TODO: TLSH_OPTION_THREADED | TLSH_OPTION_PRIVATE

        let mut fed_len = self.data_len;

        // XXX: this code has been tweaked compared to the C++ version, to improve performances.
        // See <https://github.com/vthib/tlsh/pull/8>.
        for b in data {
            let b_0 = *b;
            let [b_4, b_3, b_2, b_1] = self.slide_window;

            if fed_len >= 4 {
                for k in 0..TLSH_CHECKSUM_LEN {
                    if k == 0 {
                        self.checksum[k] =
                            fast_b_mapping::<EFF_BUCKETS>(1, b_0, b_1, self.checksum[k]);
                    } else {
                        self.checksum[k] =
                            b_mapping(self.checksum[k - 1], b_0, b_1, self.checksum[k]);
                    }
                }

                let r = fast_b_mapping::<EFF_BUCKETS>(49, b_0, b_1, b_2);
                self.a_bucket[usize::from(r)] += 1;
                let r = fast_b_mapping::<EFF_BUCKETS>(12, b_0, b_1, b_3);
                self.a_bucket[usize::from(r)] += 1;
                let r = fast_b_mapping::<EFF_BUCKETS>(84, b_0, b_1, b_4);
                self.a_bucket[usize::from(r)] += 1;
                let r = fast_b_mapping::<EFF_BUCKETS>(178, b_0, b_2, b_3);
                self.a_bucket[usize::from(r)] += 1;
                let r = fast_b_mapping::<EFF_BUCKETS>(166, b_0, b_2, b_4);
                self.a_bucket[usize::from(r)] += 1;
                let r = fast_b_mapping::<EFF_BUCKETS>(230, b_0, b_3, b_4);
                self.a_bucket[usize::from(r)] += 1;
            }
            fed_len += 1;
            self.slide_window = [b_3, b_2, b_1, b_0];
        }

        self.data_len += data.len();
    }

    /// Generate a [`Tlsh`] object, or None if the object is not valid.
    pub fn build(&self) -> Option<Tlsh<TLSH_CHECKSUM_LEN, TLSH_STRING_LEN_REQ, CODE_SIZE>> {
        if self.data_len < MIN_DATA_LENGTH {
            return None;
        }

        let (q1, q2, q3) = get_quartiles::<EFF_BUCKETS>(&self.a_bucket);
        // issue #79 - divide by 0 if q3 == 0
        if q3 == 0 {
            return None;
        }

        // buckets must be more than 50% non-zero
        let nonzero = self
            .a_bucket
            .iter()
            .take(CODE_SIZE * 4)
            .filter(|v| **v > 0)
            .count();
        if EFF_BUCKETS == 48 {
            if nonzero < 18 {
                return None;
            }
        } else if nonzero <= 2 * CODE_SIZE {
            return None;
        }

        // Safety: this expect is eliminated at compile time, as the compiler can
        // trivially verify that EFF_BUCKETS <= BUCKETS.
        let bucket: &[u32; EFF_BUCKETS] = (&self.a_bucket[..EFF_BUCKETS])
            .try_into()
            .expect("EFF_BUCKETS is bigger than BUCKETS");

        let mut code: [u8; CODE_SIZE] = [0; CODE_SIZE];
        for (c, slice) in code.iter_mut().zip(bucket.chunks(4)) {
            *c = slice.iter().rev().fold(0u8, |h, &k| {
                (if q3 < k {
                    3
                } else if q2 < k {
                    2
                } else if q1 < k {
                    1
                } else {
                    0
                }) | h << 2
            });
        }

        let lvalue = l_capturing(self.data_len as u32);
        let q1_ratio = (((((q1 * 100) as f32) / (q3 as f32)) as u32) % 16) as u8;
        let q2_ratio = (((((q2 * 100) as f32) / (q3 as f32)) as u32) % 16) as u8;

        Some(Tlsh {
            lvalue,
            q1_ratio,
            q2_ratio,
            checksum: self.checksum,
            code,
        })
    }
}

/// TLSH object, from which a hash or a distance can be computed.
pub struct Tlsh<
    const TLSH_CHECKSUM_LEN: usize,
    const TLSH_STRING_LEN_REQ: usize,
    const CODE_SIZE: usize,
> {
    lvalue: u8,
    q1_ratio: u8,
    q2_ratio: u8,
    checksum: [u8; TLSH_CHECKSUM_LEN],
    code: [u8; CODE_SIZE],
}

impl<const TLSH_CHECKSUM_LEN: usize, const TLSH_STRING_LEN_REQ: usize, const CODE_SIZE: usize>
    Tlsh<TLSH_CHECKSUM_LEN, TLSH_STRING_LEN_REQ, CODE_SIZE>
{
    /// Compute the hash of a TLSH.
    ///
    /// The hash is always prefixed by `T1` (`showvers=1` in the original TLSH version).
    /// This is due to the no_std implementation and the need to have a fixed-length result.
    /// Use a subslice on the result if you don't need this prefix.
    ///
    /// ```
    /// let data = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit";
    /// let tlsh = tlsh2::TlshDefaultBuilder::build_from(data)
    ///     .expect("should have generated a TLSH");
    /// assert_eq!(
    ///     tlsh.hash().as_slice(),
    ///     b"T12D900249414E0BD59A46503F3ADA802AE50825242B2590561CF690599112214C051556",
    /// );
    /// ```
    pub fn hash(&self) -> [u8; TLSH_STRING_LEN_REQ] {
        let mut hash = [0; TLSH_STRING_LEN_REQ];

        hash[0] = b'T';
        hash[1] = b'1';
        let mut i = 2;

        for k in &self.checksum {
            to_hex(&mut hash, &mut i, swap_byte(*k));
        }
        to_hex(&mut hash, &mut i, swap_byte(self.lvalue));

        let qb = (self.q1_ratio << 4) | self.q2_ratio;
        to_hex(&mut hash, &mut i, qb);

        for c in self.code.iter().rev() {
            to_hex(&mut hash, &mut i, *c);
        }

        hash
    }

    /// Compute the difference between two TLSH.
    ///
    /// The len_diff parameter specifies if the file length is to be included in
    /// the difference calculation (len_diff=true) or if it is to be excluded
    /// (len_diff=false).
    ///
    /// In general, the length should be considered in the difference calculation,
    /// but there could be applications where a part of the adversarial activity
    /// might be to add a lot of content.
    /// For example to add 1 million zero bytes at the end of a file. In that case,
    /// the caller would want to exclude the length from the calculation.
    ///
    /// ```
    /// let data1 = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit";
    /// let tlsh1 = tlsh2::TlshDefaultBuilder::build_from(data1)
    ///     .expect("should have generated a TLSH");
    /// let data2 = b"Duis aute irure dolor in reprehenderit in voluptate velit \
    ///     esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat \
    ///     cupidatat non proident, sunt in culpa qui officia";
    /// let tlsh2 = tlsh2::TlshDefaultBuilder::build_from(data2)
    ///     .expect("should have generated a TLSH");
    ///
    /// assert_eq!(tlsh1.diff(&tlsh2, false), 244);
    /// assert_eq!(tlsh1.diff(&tlsh2, true), 280);
    /// ```
    #[cfg(feature = "diff")]
    pub fn diff(&self, other: &Self, len_diff: bool) -> i32 {
        use crate::util::{h_distance, mod_diff};

        const LENGTH_MULT: i32 = 12;
        const QRATIO_MULT: i32 = 12;
        const RANGE_LVALUE: u32 = 256;
        const RANGE_QRATIO: u32 = 16;

        let mut diff;
        if len_diff {
            let ldiff = mod_diff(self.lvalue, other.lvalue, RANGE_LVALUE);
            if ldiff == 0 {
                diff = 0;
            } else if ldiff == 1 {
                diff = 1;
            } else {
                diff = ldiff * LENGTH_MULT;
            }
        } else {
            diff = 0;
        }

        let q1diff = mod_diff(self.q1_ratio, other.q1_ratio, RANGE_QRATIO);
        if q1diff <= 1 {
            diff += q1diff;
        } else {
            diff += (q1diff - 1) * QRATIO_MULT;
        }

        let q2diff = mod_diff(self.q2_ratio, other.q2_ratio, RANGE_QRATIO);
        if q2diff <= 1 {
            diff += q2diff;
        } else {
            diff += (q2diff - 1) * QRATIO_MULT;
        }

        for (a, b) in self.checksum.iter().zip(other.checksum.iter()) {
            if a != b {
                diff += 1;
                break;
            }
        }

        diff += h_distance(&self.code, &other.code);

        diff
    }

    fn from_hash(s: &[u8]) -> Option<Self> {
        if s.len() != TLSH_STRING_LEN_REQ || s[0] != b'T' || s[1] != b'1' {
            return None;
        }

        let mut i = 2;

        let mut checksum = [0; TLSH_CHECKSUM_LEN];
        for k in &mut checksum {
            *k = swap_byte(from_hex(s, &mut i)?);
        }

        let lvalue = swap_byte(from_hex(s, &mut i)?);
        let qb = from_hex(s, &mut i)?;
        let q1_ratio = qb >> 4;
        let q2_ratio = qb & 0x0F;

        let mut code = [0; CODE_SIZE];
        for c in code.iter_mut().rev() {
            *c = from_hex(s, &mut i)?;
        }

        Some(Self {
            lvalue,
            q1_ratio,
            q2_ratio,
            checksum,
            code,
        })
    }
}

/// Error returned when failing to convert a hash string to a `Tlsh` object.
#[derive(Debug, PartialEq, Eq)]
pub struct ParseError;

/// Parse a hash string and build the corresponding `Tlsh` object.
impl<const TLSH_CHECKSUM_LEN: usize, const TLSH_STRING_LEN_REQ: usize, const CODE_SIZE: usize>
    FromStr for Tlsh<TLSH_CHECKSUM_LEN, TLSH_STRING_LEN_REQ, CODE_SIZE>
{
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_hash(s.as_bytes()).ok_or(ParseError)
    }
}

fn from_hex(s: &[u8], i: &mut usize) -> Option<u8> {
    let a = char::from(s[*i]).to_digit(16)?;
    *i += 1;
    let b = char::from(s[*i]).to_digit(16)?;
    *i += 1;

    Some(((a as u8) << 4) | (b as u8))
}

fn to_hex(s: &mut [u8], s_idx: &mut usize, b: u8) {
    const HEX_LOOKUP: &[u8] = b"000102030405060708090A0B0C0D0E0F\
    101112131415161718191A1B1C1D1E1F\
    202122232425262728292A2B2C2D2E2F\
    303132333435363738393A3B3C3D3E3F\
    404142434445464748494A4B4C4D4E4F\
    505152535455565758595A5B5C5D5E5F\
    606162636465666768696A6B6C6D6E6F\
    707172737475767778797A7B7C7D7E7F\
    808182838485868788898A8B8C8D8E8F\
    909192939495969798999A9B9C9D9E9F\
    A0A1A2A3A4A5A6A7A8A9AAABACADAEAF\
    B0B1B2B3B4B5B6B7B8B9BABBBCBDBEBF\
    C0C1C2C3C4C5C6C7C8C9CACBCCCDCECF\
    D0D1D2D3D4D5D6D7D8D9DADBDCDDDEDF\
    E0E1E2E3E4E5E6E7E8E9EAEBECEDEEEF\
    F0F1F2F3F4F5F6F7F8F9FAFBFCFDFEFF";

    let i = usize::from(b) * 2;
    s[*s_idx] = HEX_LOOKUP[i];
    s[*s_idx + 1] = HEX_LOOKUP[i + 1];
    *s_idx += 2;
}
