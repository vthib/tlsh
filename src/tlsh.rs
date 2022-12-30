use crate::pearson::{b_mapping, fast_b_mapping};
use crate::quartile::get_quartiles;
use crate::util::{l_capturing, swap_byte};

const SLIDING_WND_SIZE: usize = 5;
const BUCKETS: usize = 256;

const RNG_SIZE: usize = SLIDING_WND_SIZE;

/// Core TLSH hasher, generic on several parameters.
///
/// You should never provide your own values for the generics, but instead use the pre-configured
/// types such as [`crate::Tlsh256_1`] or [`crate::Tlsh128_3`].
pub struct TlshCore<
    const EFF_BUCKETS: usize,
    const TLSH_CHECKSUM_LEN: usize,
    const CODE_SIZE: usize,
    const TLSH_STRING_LEN_REQ: usize,
    const MIN_DATA_LENGTH: usize,
> {
    a_bucket: [u32; BUCKETS],
    slide_window: [u8; SLIDING_WND_SIZE],
    checksum: [u8; 3],
    data_len: usize,
}

impl<
        const EFF_BUCKETS: usize,
        const TLSH_CHECKSUM_LEN: usize,
        const CODE_SIZE: usize,
        const TLSH_STRING_LEN_REQ: usize,
        const MIN_DATA_LENGTH: usize,
    > Default
    for TlshCore<EFF_BUCKETS, TLSH_CHECKSUM_LEN, CODE_SIZE, TLSH_STRING_LEN_REQ, MIN_DATA_LENGTH>
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
    > TlshCore<EFF_BUCKETS, TLSH_CHECKSUM_LEN, CODE_SIZE, TLSH_STRING_LEN_REQ, MIN_DATA_LENGTH>
{
    /// Create a new TLSH hasher.
    pub fn new() -> Self {
        Self {
            a_bucket: [0; BUCKETS],
            slide_window: [0; SLIDING_WND_SIZE],
            checksum: [0; 3],
            data_len: 0,
        }
    }

    /// Add bytes into the hasher.
    pub fn update(&mut self, data: &[u8]) {
        // TODO: TLSH_CHECKSUM_LEN
        // TODO: TLSH_OPTION_THREADED | TLSH_OPTION_PRIVATE

        let mut j = self.data_len % RNG_SIZE;
        let mut fed_len = self.data_len;

        for b in data {
            self.slide_window[j] = *b;

            if fed_len >= 4 {
                // Only wrote code valid for SLIDING_WND_SIZE == 5, so assert it.
                // Need to align to orig impl if this value changes.
                static_assertions::const_assert_eq!(SLIDING_WND_SIZE, 5);
                let j_1 = (j + RNG_SIZE - 1) % RNG_SIZE;
                let j_2 = (j + RNG_SIZE - 2) % RNG_SIZE;
                let j_3 = (j + RNG_SIZE - 3) % RNG_SIZE;
                let j_4 = (j + RNG_SIZE - 4) % RNG_SIZE;

                // TODO: CHECKSUM_0B
                for k in 0..TLSH_CHECKSUM_LEN {
                    if k == 0 {
                        self.checksum[k] = fast_b_mapping::<EFF_BUCKETS>(
                            1,
                            self.slide_window[j],
                            self.slide_window[j_1],
                            self.checksum[k],
                        );
                    } else {
                        self.checksum[k] = b_mapping(
                            self.checksum[k - 1],
                            self.slide_window[j],
                            self.slide_window[j_1],
                            self.checksum[k],
                        );
                    }
                }

                let r = fast_b_mapping::<EFF_BUCKETS>(
                    49,
                    self.slide_window[j],
                    self.slide_window[j_1],
                    self.slide_window[j_2],
                );
                self.a_bucket[usize::from(r)] += 1;
                let r = fast_b_mapping::<EFF_BUCKETS>(
                    12,
                    self.slide_window[j],
                    self.slide_window[j_1],
                    self.slide_window[j_3],
                );
                self.a_bucket[usize::from(r)] += 1;
                let r = fast_b_mapping::<EFF_BUCKETS>(
                    178,
                    self.slide_window[j],
                    self.slide_window[j_2],
                    self.slide_window[j_3],
                );
                self.a_bucket[usize::from(r)] += 1;
                let r = fast_b_mapping::<EFF_BUCKETS>(
                    166,
                    self.slide_window[j],
                    self.slide_window[j_2],
                    self.slide_window[j_4],
                );
                self.a_bucket[usize::from(r)] += 1;
                let r = fast_b_mapping::<EFF_BUCKETS>(
                    84,
                    self.slide_window[j],
                    self.slide_window[j_1],
                    self.slide_window[j_4],
                );
                self.a_bucket[usize::from(r)] += 1;
                let r = fast_b_mapping::<EFF_BUCKETS>(
                    230,
                    self.slide_window[j],
                    self.slide_window[j_3],
                    self.slide_window[j_4],
                );
                self.a_bucket[usize::from(r)] += 1;
            }
            fed_len += 1;
            j = (j + 1) % RNG_SIZE;
        }

        self.data_len += data.len();
    }

    /// Finish the hashing, returning the TLSH hash string.
    ///
    /// if `showvers` is true, the hash is prefixed with the string `T1`.
    pub fn finish(self, showvers: bool) -> String {
        if self.data_len < MIN_DATA_LENGTH {
            return String::new();
        }

        let (q1, q2, q3) = get_quartiles::<EFF_BUCKETS>(&self.a_bucket);
        // issue #79 - divide by 0 if q3 == 0
        if q3 == 0 {
            return String::new();
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
                return String::new();
            }
        } else if nonzero <= 2 * CODE_SIZE {
            return String::new();
        }

        let code: Vec<u8> = self
            .a_bucket
            .chunks(4)
            .take(CODE_SIZE)
            .map(|slice| {
                let mut h = 0_u8;
                for (j, k) in slice.iter().enumerate() {
                    if q3 < *k {
                        h += 3 << (j * 2);
                    } else if q2 < *k {
                        h += 2 << (j * 2);
                    } else if q1 < *k {
                        h += 1 << (j * 2);
                    }
                }
                h
            })
            .collect();

        let lvalue = l_capturing(self.data_len as u32);
        let q1_ratio = ((((q1 * 100) as f32) / (q3 as f32)) as u32) % 16;
        let q2_ratio = ((((q2 * 100) as f32) / (q3 as f32)) as u32) % 16;
        // TODO: is there an endianness issue here?
        let qb = ((q2_ratio << 4) as u8) | (q1_ratio as u8);

        hash::<TLSH_STRING_LEN_REQ>(
            &self.checksum[..TLSH_CHECKSUM_LEN],
            lvalue,
            qb,
            &code,
            showvers,
        )
    }
}

fn hash<const TLSH_STRING_LEN_REQ: usize>(
    checksum: &[u8],
    lvalue: u8,
    qb: u8,
    code: &[u8],
    showvers: bool,
) -> String {
    let mut hash = String::with_capacity(TLSH_STRING_LEN_REQ);

    if showvers {
        hash.push_str("T1");
    }

    for k in checksum {
        to_hex(&mut hash, swap_byte(*k));
    }
    to_hex(&mut hash, swap_byte(lvalue));
    to_hex(&mut hash, swap_byte(qb));
    for c in code.iter().rev() {
        to_hex(&mut hash, *c);
    }

    hash
}

fn to_hex(s: &mut String, b: u8) {
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
    s.push(char::from(HEX_LOOKUP[i]));
    s.push(char::from(HEX_LOOKUP[i + 1]));
}
