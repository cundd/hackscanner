#![allow(unused_attributes)]
#![feature(test)]
extern crate hackscanner_lib;
extern crate test;

use hackscanner_lib::file_finder::FileFinderTrait;
use hackscanner_lib::*;
use test::Bencher;

fn get_test_dir() -> String {
    format!("{}/tests", env!("CARGO_MANIFEST_DIR"))
}

pub fn bench_find_files<D, F>(ff: &F, b: &mut Bencher)
where
    D: DirEntryTrait,
    F: FileFinderTrait<DirEntry = D>,
{
    let rules = vec![Rule::new(
        "2",
        Severity::NOTICE,
        "\\.tx_mocfilemanager",
        true,
        None,
    )];
    b.iter(|| ff.find(get_test_dir(), &rules));
}

pub fn bench_find_files_without_rules<D, F>(ff: &F, b: &mut Bencher)
where
    D: DirEntryTrait,
    F: FileFinderTrait<DirEntry = D>,
{
    b.iter(|| ff.find(get_test_dir(), &vec![]));
}
