#![feature(test)]
extern crate test;
extern crate hackscanner_lib;

mod fts {
    use test::Bencher;
    use hackscanner_lib::*;
    use hackscanner_lib::file_finder::*;
    use hackscanner_lib::file_finder::walkdir::FileFinder;

    #[bench]
    fn bench_find_files(b: &mut Bencher) {
        let rules = vec![Rule::new("2".to_string(), Severity::NOTICE, Some("\\.tx_mocfilemanager".to_owned()), None)];
        let ff = FileFinder::new();
        b.iter(|| ff.find(env!("CARGO_MANIFEST_DIR"), &rules));
    }

    #[bench]
    fn bench_find_files_without_rules(b: &mut Bencher) {
        let ff = FileFinder::new();
        b.iter(|| ff.find(env!("CARGO_MANIFEST_DIR"), &vec![]));
    }
}
