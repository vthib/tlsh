use std::collections::HashMap;
use std::io::BufRead;
use std::path::Path;

#[test]
fn test_conformance_files() {
    // Build conformance results map
    let f = std::fs::File::open("tests/assets/example_data.out").unwrap();
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
            .get(&dbg!(format!(
                "example_data/{}",
                Path::new(file.file_name().unwrap()).display()
            )))
            .unwrap();

        // Test the rust crate computes the same
        {
            let mut tlsh = tlsh::Tlsh::new();
            tlsh.update(&contents);
            assert_eq!(&tlsh.finish(true), expected_hash);
        }
    }
}
