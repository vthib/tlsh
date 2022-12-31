use std::collections::HashMap;
use std::io::BufRead;
use std::path::{Path, PathBuf};

fn test_diff<F>(path: &str, compute_diff: F)
where
    F: Fn(&Path, &Path) -> i32,
{
    let f = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(f);
    for line in reader.lines() {
        let line = line.unwrap();
        let mut line = line.split('\t');
        let path1 = line.next().unwrap();
        let path2 = line.next().unwrap();
        let expected_score = line.next().unwrap().parse::<i32>().unwrap();

        assert_eq!(
            compute_diff(Path::new(path1), Path::new(path2)),
            expected_score
        );
    }
}

fn build_cache<F, T>(f: F) -> HashMap<PathBuf, T>
where
    F: Fn(&[u8]) -> T,
{
    glob::glob("tests/assets/tlsh/example_data/*")
        .unwrap()
        .map(|path| {
            let path = path.unwrap();
            let contents = std::fs::read(&path).unwrap();
            (
                path.strip_prefix("tests/assets/tlsh")
                    .unwrap()
                    .to_path_buf(),
                f(&contents),
            )
        })
        .collect()
}

macro_rules! do_diff_test {
    ($testname:ident, $name:expr, $type:ty, $len_diff:expr) => {
        #[test]
        fn $testname() {
            let cache = build_cache(|contents| {
                let mut tlsh = <$type>::new();
                tlsh.update(contents);
                tlsh.build().unwrap()
            });

            test_diff(
                &format!(
                    "tests/assets/tlsh/exp/example_data.{}.xref.scores_EXP",
                    $name
                ),
                |path1, path2| {
                    let tlsh1 = cache.get(path1).unwrap();
                    let tlsh2 = cache.get(path2).unwrap();
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
