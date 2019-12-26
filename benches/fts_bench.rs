#![feature(test)]
extern crate hackscanner_lib;
extern crate test;

mod bench_helper;

mod fts {
    use super::*;
    use hackscanner_lib::file_finder::walkdir::FileFinder;
    use test::Bencher;

    #[bench]
    fn bench_find_files(b: &mut Bencher) {
        bench_helper::bench_find_files(&FileFinder::new(), b);
    }

    #[bench]
    fn bench_find_files_without_rules(b: &mut Bencher) {
        bench_helper::bench_find_files_without_rules(&FileFinder::new(), b);
    }
}
