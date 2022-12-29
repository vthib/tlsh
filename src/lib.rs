mod pearson;
mod quartile;
mod tlsh;
mod util;

pub type Tlsh256_1 = crate::tlsh::Tlsh<256, 1, 64, 136, 50>;
pub type Tlsh128_1 = crate::tlsh::Tlsh<128, 1, 32, 72, 50>;
pub type Tlsh48_1 = crate::tlsh::Tlsh<48, 1, 12, 30, 10>;

pub type Tlsh256_3 = crate::tlsh::Tlsh<256, 3, 64, 136, 50>;
pub type Tlsh128_3 = crate::tlsh::Tlsh<128, 3, 32, 72, 50>;

pub type Tlsh = Tlsh128_1;
