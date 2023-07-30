use std::io::BufRead;

fn test_hash<F>(path: &str, compute_hash: F)
where
    F: Fn(&[u8]) -> String,
{
    let f = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(f);
    for line in reader.lines() {
        let line = line.unwrap();
        let mut line = line.split('\t');
        let expected_hash = line.next().unwrap();
        let filepath = line.next().unwrap();

        let fullpath = format!("tests/assets/tlsh/{filepath}");
        let contents = std::fs::read(&fullpath)
            .unwrap_or_else(|e| panic!("cannot read file {fullpath:?}: {e}"));

        assert_eq!(&compute_hash(&contents), expected_hash);
    }
}

macro_rules! do_hash_test {
    ($testname:ident, $name:expr, $builder:ty, $tlsh:ty) => {
        #[test]
        fn $testname() {
            test_hash(
                &format!("tests/assets/tlsh/exp/example_data.{}.len.out_EXP", $name),
                |contents| {
                    let mut builder = <$builder>::new();
                    builder.update(contents);
                    let tlsh = builder.build().unwrap();
                    let hash = String::from_utf8(tlsh.hash().to_vec()).unwrap();

                    // Test the FromStr implementation
                    let tlsh2 = hash.parse::<$tlsh>().unwrap();
                    assert_eq!(tlsh.hash(), tlsh2.hash());

                    hash
                },
            )
        }
    };
}

do_hash_test!(
    test_hash_48_1,
    "48.1",
    tlsh2::TlshBuilder48_1,
    tlsh2::Tlsh48_1
);
do_hash_test!(
    test_hash_128_1,
    "128.1",
    tlsh2::TlshBuilder128_1,
    tlsh2::Tlsh128_1
);
do_hash_test!(
    test_hash_128_3,
    "128.3",
    tlsh2::TlshBuilder128_3,
    tlsh2::Tlsh128_3
);
do_hash_test!(
    test_hash_256_1,
    "256.1",
    tlsh2::TlshBuilder256_1,
    tlsh2::Tlsh256_1
);
do_hash_test!(
    test_hash_256_3,
    "256.3",
    tlsh2::TlshBuilder256_3,
    tlsh2::Tlsh256_3
);
