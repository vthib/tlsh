use std::collections::HashMap;
use std::io::BufRead;
use std::path::Path;

fn test<F>(path: &str, compute_hash: F)
where
    F: Fn(&[u8]) -> String,
{
    // Build conformance results map
    let f = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(f);
    let expected_results = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut line = line.split('\t');
            let expected_hash = line.next().unwrap();
            let filepath = line.next().unwrap();

            (filepath.to_owned(), expected_hash.to_owned())
        })
        .collect::<HashMap<_, _>>();

    for file in glob::glob("tests/assets/tlsh/example_data/*").unwrap() {
        let file = file.unwrap();
        let contents =
            std::fs::read(&file).unwrap_or_else(|e| panic!("cannot read file {:?}: {}", file, e));
        let expected_hash = expected_results
            .get(&format!(
                "example_data/{}",
                Path::new(file.file_name().unwrap()).display()
            ))
            .unwrap();

        assert_eq!(&compute_hash(&contents), expected_hash);
    }
}

macro_rules! do_test {
    ($path:expr, $type:ty) => {{
        test($path, |contents| {
            let mut tlsh = <$type>::new();
            tlsh.update(contents);
            tlsh.finish(true)
        })
    }};
}

#[test]
fn test_conformance_file_48_1() {
    do_test!("tests/assets/example_data.out.48.1", tlsh2::Tlsh48_1);
}

#[test]
fn test_conformance_file_128_1() {
    do_test!("tests/assets/example_data.out.128.1", tlsh2::Tlsh128_1);
}

#[test]
fn test_conformance_file_128_3() {
    do_test!("tests/assets/example_data.out.128.3", tlsh2::Tlsh128_3);
}

#[test]
fn test_conformance_file_256_1() {
    do_test!("tests/assets/example_data.out.256.1", tlsh2::Tlsh256_1);
}

#[test]
fn test_conformance_file_256_3() {
    do_test!("tests/assets/example_data.out.256.3", tlsh2::Tlsh256_3);
}
