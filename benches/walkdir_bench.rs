#![feature(test)]
extern crate test;
extern crate hackscanner_lib;

use test::Bencher;
use hackscanner_lib::*;
use file_finder::*;

mod walkdir {
    use super::*;

    #[bench]
    fn bench_find_files_test(b: &mut Bencher) {
        let rules = vec![Rule::new("2".to_string(), Severity::NOTICE, Some("\\.tx_mocfilemanager".to_owned()), None)];
        b.iter(|| file_finder::walkdir::FileFinder::find(env!("CARGO_MANIFEST_DIR"), &rules));
    }
}
