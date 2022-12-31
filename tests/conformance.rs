use std::io::BufRead;

fn test<F>(path: &str, compute_hash: F)
where
    F: Fn(&[u8]) -> String,
{
    let f = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(f);
    for line in reader.lines() {
        let line = line.unwrap();
        dbg!(&line);
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
        test($path, |contents| {
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
