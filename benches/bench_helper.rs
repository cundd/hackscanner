#![allow(unused_attributes)]
#![feature(test)]
extern crate test;
extern crate hackscanner_lib;

use test::Bencher;
use hackscanner_lib::file_finder::FileFinderTrait;
use hackscanner_lib::*;

fn get_test_dir() -> String {
    format!("{}/tests", env!("CARGO_MANIFEST_DIR"))
}

pub fn bench_find_files<D, F>(ff: &F, b: &mut Bencher)
    where D: DirEntryTrait, F: FileFinderTrait<DirEntry=D> {
    let rules = vec![Rule::new_raw("2".to_string(), Severity::NOTICE, Some("\\.tx_mocfilemanager".to_owned()), None)];
    b.iter(|| ff.find(get_test_dir(), &rules));
}

pub fn bench_find_files_without_rules<D, F>(ff: &F, b: &mut Bencher)
    where D: DirEntryTrait, F: FileFinderTrait<DirEntry=D> {
    b.iter(|| ff.find(get_test_dir(), &vec![]));
}
