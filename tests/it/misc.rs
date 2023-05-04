pub fn check(bytes: &[u8], expected_hash: &[u8]) {
    let mut tlsh = tlsh2::TlshDefaultBuilder::new();
    tlsh.update(bytes);
    assert_eq!(
        tlsh.build().map(|v| v.hash().to_vec()).unwrap_or_default(),
        expected_hash
    );
}

fn times(bytes: &[u8], times: u32) -> Vec<u8> {
    let mut res = Vec::new();

    for _ in 0..times {
        res.extend(bytes);
    }
    res
}

// Test that a long payload does not trigger overflows.
//
// This tests that the `l_capturing` helper works properly when the search is in the second half of
// the array.
#[test]
fn test_regression_1() {
    // 64B
    let str1 = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do ";

    // 1MB
    check(
        &times(str1, 16384),
        b"T1CE25220E008F0BC28F03003F3EEBC0AEE00820A0AB2080220CF2C02C8002208C002802",
    );

    // 5MB
    check(
        &times(str1, 16384 * 5),
        b"T15136220E008F0BC28F03003F3EEBC0AEE00820A0AB2080220CF2C02C8002208C002802",
    );

    // 10MB
    check(
        &times(str1, 16384 * 10),
        b"T146B6220E008F0BC28F03003F3EEBC0AEE00820A0AB2080220CF2C02C8002208C002802",
    );
}
