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
    ($path:expr, $type:ty) => {{
        test_hash($path, |contents| {
            let mut tlsh = <$type>::new();
            tlsh.update(contents);
            tlsh.finish(true)
        })
    }};
}

#[test]
fn test_hash_48_1() {
    do_hash_test!(
        "tests/assets/tlsh/exp/example_data.48.1.len.out_EXP",
        tlsh2::Tlsh48_1
    );
}

#[test]
fn test_hash_128_1() {
    do_hash_test!(
        "tests/assets/tlsh/exp/example_data.128.1.len.out_EXP",
        tlsh2::Tlsh128_1
    );
}

#[test]
fn test_hash_128_3() {
    do_hash_test!(
        "tests/assets/tlsh/exp/example_data.128.3.len.out_EXP",
        tlsh2::Tlsh128_3
    );
}

#[test]
fn test_hash_256_1() {
    do_hash_test!(
        "tests/assets/tlsh/exp/example_data.256.1.len.out_EXP",
        tlsh2::Tlsh256_1
    );
}

#[test]
fn test_hash_256_3() {
    do_hash_test!(
        "tests/assets/tlsh/exp/example_data.256.3.len.out_EXP",
        tlsh2::Tlsh256_3
    );
}

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
    ($path:expr, $type:ty, $len_diff:expr) => {{
        test_diff($path, |contents1, contents2| {
            let mut tlsh1 = <$type>::new();
            tlsh1.update(contents1);
            let mut tlsh2 = <$type>::new();
            tlsh2.update(contents2);
            tlsh1.diff(&tlsh2, $len_diff)
        })
    }};
}

#[test]
fn test_diff_48_1_len() {
    do_diff_test!(
        "tests/assets/tlsh/exp/example_data.48.1.len.xref.scores_EXP",
        tlsh2::Tlsh48_1,
        true
    );
}

#[test]
fn test_diff_48_1_xlen() {
    do_diff_test!(
        "tests/assets/tlsh/exp/example_data.48.1.xlen.xref.scores_EXP",
        tlsh2::Tlsh48_1,
        false
    );
}

#[test]
fn test_diff_128_1_len() {
    do_diff_test!(
        "tests/assets/tlsh/exp/example_data.128.1.len.xref.scores_EXP",
        tlsh2::Tlsh128_1,
        true
    );
}

#[test]
fn test_diff_128_1_xlen() {
    do_diff_test!(
        "tests/assets/tlsh/exp/example_data.128.1.xlen.xref.scores_EXP",
        tlsh2::Tlsh128_1,
        false
    );
}

#[test]
fn test_diff_128_3_len() {
    do_diff_test!(
        "tests/assets/tlsh/exp/example_data.128.3.len.xref.scores_EXP",
        tlsh2::Tlsh128_3,
        true
    );
}

#[test]
fn test_diff_128_3_xlen() {
    do_diff_test!(
        "tests/assets/tlsh/exp/example_data.128.3.xlen.xref.scores_EXP",
        tlsh2::Tlsh128_3,
        false
    );
}

#[test]
fn test_diff_256_1_len() {
    do_diff_test!(
        "tests/assets/tlsh/exp/example_data.256.1.len.xref.scores_EXP",
        tlsh2::Tlsh256_1,
        true
    );
}

#[test]
fn test_diff_256_1_xlen() {
    do_diff_test!(
        "tests/assets/tlsh/exp/example_data.256.1.xlen.xref.scores_EXP",
        tlsh2::Tlsh256_1,
        false
    );
}

#[test]
fn test_diff_256_3_len() {
    do_diff_test!(
        "tests/assets/tlsh/exp/example_data.256.3.len.xref.scores_EXP",
        tlsh2::Tlsh256_3,
        true
    );
}

#[test]
fn test_diff_256_3_xlen() {
    do_diff_test!(
        "tests/assets/tlsh/exp/example_data.256.3.xlen.xref.scores_EXP",
        tlsh2::Tlsh256_3,
        false
    );
}
