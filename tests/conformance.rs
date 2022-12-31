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

        let fullpath = format!("tests/assets/tlsh/{}", filepath);
        let contents = std::fs::read(&fullpath)
            .unwrap_or_else(|e| panic!("cannot read file {:?}: {}", fullpath, e));

        assert_eq!(&compute_hash(&contents), expected_hash);
    }
}

macro_rules! do_hash_test {
    ($testname:ident, $name:expr, $type:ty) => {
        #[test]
        fn $testname() {
            test_hash(
                &format!("tests/assets/tlsh/exp/example_data.{}.len.out_EXP", $name),
                |contents| {
                    let mut tlsh = <$type>::new();
                    tlsh.update(contents);
                    tlsh.finish(true)
                },
            )
        }
    };
}

do_hash_test!(test_hash_48_1, "48.1", tlsh2::Tlsh48_1);
do_hash_test!(test_hash_128_1, "128.1", tlsh2::Tlsh128_1);
do_hash_test!(test_hash_128_3, "128.3", tlsh2::Tlsh128_3);
do_hash_test!(test_hash_256_1, "256.1", tlsh2::Tlsh256_1);
do_hash_test!(test_hash_256_3, "256.3", tlsh2::Tlsh256_3);

fn test_diff<F>(path: &str, compute_diff: F)
where
    F: Fn(&[u8], &[u8]) -> i32,
{
    let f = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(f);
    for line in reader.lines() {
        let line = line.unwrap();
        let mut line = line.split('\t');
        let path1 = line.next().unwrap();
        let path2 = line.next().unwrap();
        let expected_score = line.next().unwrap().parse::<i32>().unwrap();

        let path1 = format!("tests/assets/tlsh/{}", path1);
        let path2 = format!("tests/assets/tlsh/{}", path2);

        let contents1 =
            std::fs::read(&path1).unwrap_or_else(|e| panic!("cannot read file {:?}: {}", path1, e));
        let contents2 =
            std::fs::read(&path2).unwrap_or_else(|e| panic!("cannot read file {:?}: {}", path2, e));

        assert_eq!(compute_diff(&contents1, &contents2), expected_score);
    }
}

macro_rules! do_diff_test {
    ($testname:ident, $name:expr, $type:ty, $len_diff:expr) => {
        #[test]
        fn $testname() {
            test_diff(
                &format!(
                    "tests/assets/tlsh/exp/example_data.{}.xref.scores_EXP",
                    $name
                ),
                |contents1, contents2| {
                    let mut tlsh1 = <$type>::new();
                    tlsh1.update(contents1);
                    let mut tlsh2 = <$type>::new();
                    tlsh2.update(contents2);
                    tlsh1.diff(&tlsh2, $len_diff)
                },
            )
        }
    };
}

do_diff_test!(test_diff_48_1_len, "48.1.len", tlsh2::Tlsh48_1, true);
do_diff_test!(test_diff_48_1_xlen, "48.1.xlen", tlsh2::Tlsh48_1, false);
do_diff_test!(test_diff_128_1_len, "128.1.len", tlsh2::Tlsh128_1, true);
do_diff_test!(test_diff_128_1_xlen, "128.1.xlen", tlsh2::Tlsh128_1, false);
do_diff_test!(test_diff_128_3_len, "128.3.len", tlsh2::Tlsh128_3, true);
do_diff_test!(test_diff_128_3_xlen, "128.3.xlen", tlsh2::Tlsh128_3, false);
do_diff_test!(test_diff_256_1_len, "256.1.len", tlsh2::Tlsh256_1, true);
do_diff_test!(test_diff_256_1_xlen, "256.1.xlen", tlsh2::Tlsh256_1, false);
do_diff_test!(test_diff_256_3_len, "256.3.len", tlsh2::Tlsh256_3, true);
do_diff_test!(test_diff_256_3_xlen, "256.3.xlen", tlsh2::Tlsh256_3, false);
