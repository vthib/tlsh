// Pearson's sample random table
static V_TABLE: [u8; 256] = [
    1, 87, 49, 12, 176, 178, 102, 166, 121, 193, 6, 84, 249, 230, 44, 163, 14, 197, 213, 181, 161,
    85, 218, 80, 64, 239, 24, 226, 236, 142, 38, 200, 110, 177, 104, 103, 141, 253, 255, 50, 77,
    101, 81, 18, 45, 96, 31, 222, 25, 107, 190, 70, 86, 237, 240, 34, 72, 242, 20, 214, 244, 227,
    149, 235, 97, 234, 57, 22, 60, 250, 82, 175, 208, 5, 127, 199, 111, 62, 135, 248, 174, 169,
    211, 58, 66, 154, 106, 195, 245, 171, 17, 187, 182, 179, 0, 243, 132, 56, 148, 75, 128, 133,
    158, 100, 130, 126, 91, 13, 153, 246, 216, 219, 119, 68, 223, 78, 83, 88, 201, 99, 122, 11, 92,
    32, 136, 114, 52, 10, 138, 30, 48, 183, 156, 35, 61, 26, 143, 74, 251, 94, 129, 162, 63, 152,
    170, 7, 115, 167, 241, 206, 3, 150, 55, 59, 151, 220, 90, 53, 23, 131, 125, 173, 15, 238, 79,
    95, 89, 16, 105, 137, 225, 224, 217, 160, 37, 123, 118, 73, 2, 157, 46, 116, 9, 145, 134, 228,
    207, 212, 202, 215, 69, 229, 27, 188, 67, 124, 168, 252, 42, 4, 29, 108, 21, 247, 19, 205, 39,
    203, 233, 40, 186, 147, 198, 192, 155, 33, 164, 191, 98, 204, 165, 180, 117, 76, 140, 36, 210,
    172, 41, 54, 159, 8, 185, 232, 113, 196, 231, 47, 146, 120, 51, 65, 28, 144, 254, 221, 93, 189,
    194, 139, 112, 43, 71, 109, 184, 209,
];

static V_TABLE48: [u8; 256] = [
    1, 39, 1, 12, 32, 34, 6, 22, 25, 1, 6, 36, 48, 38, 44, 19, 14, 5, 21, 37, 17, 37, 26, 32, 16,
    47, 24, 34, 44, 46, 38, 8, 14, 33, 8, 7, 45, 48, 48, 2, 29, 5, 33, 18, 45, 0, 31, 30, 25, 11,
    46, 22, 38, 45, 48, 34, 24, 48, 20, 22, 48, 35, 5, 43, 1, 42, 9, 22, 12, 48, 34, 31, 16, 5, 31,
    7, 15, 14, 39, 48, 30, 25, 19, 10, 18, 10, 10, 3, 48, 27, 17, 43, 38, 35, 0, 48, 36, 8, 4, 27,
    32, 37, 14, 4, 34, 30, 43, 13, 9, 48, 24, 27, 23, 20, 31, 30, 35, 40, 9, 3, 26, 11, 44, 32, 40,
    18, 4, 10, 42, 30, 0, 39, 12, 35, 13, 26, 47, 26, 48, 46, 33, 18, 15, 8, 26, 7, 19, 23, 48, 14,
    3, 6, 7, 11, 7, 28, 42, 5, 23, 35, 29, 29, 15, 46, 31, 47, 41, 16, 9, 41, 33, 32, 25, 16, 37,
    27, 22, 25, 2, 13, 46, 20, 9, 1, 38, 36, 15, 20, 10, 23, 21, 37, 27, 44, 19, 28, 24, 48, 42, 4,
    29, 12, 21, 48, 19, 13, 39, 11, 41, 40, 42, 3, 6, 0, 11, 33, 20, 47, 2, 12, 21, 36, 21, 28, 44,
    36, 18, 28, 41, 6, 15, 8, 41, 40, 17, 4, 39, 47, 2, 24, 3, 17, 28, 0, 48, 29, 45, 45, 2, 43,
    16, 43, 23, 13, 40, 17,
];

// Two-byte lookup for Pearson's sample random table
#[cfg(feature = "fast")]
static JOINT_V_TABLE: [[u8; 256]; 256] = {
    let mut table = [[0; 256]; 256];
    let mut i = 0;
    while i < 256 {
        let mut j = 0;
        while j < 256 {
            table[i][j] = V_TABLE[V_TABLE[j] as usize ^ i];
            j += 1;
        }
        i += 1;
    }
    table
};

// Pearson's algorithm
pub fn b_mapping(salt: u8, i: u8, j: u8, k: u8) -> u8 {
    let mut h = 0;
    h = V_TABLE[usize::from(h ^ salt)];
    #[cfg(feature = "fast")]
    {
        h = JOINT_V_TABLE[usize::from(j)][usize::from(h ^ i)];
    }
    #[cfg(not(feature = "fast"))]
    {
        h = V_TABLE[usize::from(h ^ i)];
        h = V_TABLE[usize::from(h ^ j)];
    }
    h = V_TABLE[usize::from(h ^ k)];
    h
}

pub fn fast_b_mapping<const EFF_BUCKETS: usize>(salt: u8, i: u8, j: u8, k: u8) -> u8 {
    let mut h = salt;
    #[cfg(feature = "fast")]
    {
        h = JOINT_V_TABLE[usize::from(j)][usize::from(h ^ i)];
    }
    #[cfg(not(feature = "fast"))]
    {
        h = V_TABLE[usize::from(h ^ i)];
        h = V_TABLE[usize::from(h ^ j)];
    }
    if EFF_BUCKETS == 48 {
        V_TABLE48[usize::from(h ^ k)]
    } else {
        V_TABLE[usize::from(h ^ k)]
    }
}
