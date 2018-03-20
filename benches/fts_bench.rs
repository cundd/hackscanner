#![feature(test)]
extern crate test;
extern crate hackscanner_lib;

use test::Bencher;
use hackscanner_lib::*;
use file_finder::*;
use file_finder::fts::FileFinder;

mod ftw {
    use super::*;

    #[bench]
    fn bench_find_files_test(b: &mut Bencher) {
        let rules = vec![Rule::new("2".to_string(), Severity::NOTICE, Some("\\.tx_mocfilemanager".to_owned()), None)];
        let ff = FileFinder::new();
        b.iter(|| ff.find(env!("CARGO_MANIFEST_DIR"), &rules));
    }
}
