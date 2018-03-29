#![feature(test)]
extern crate test;
extern crate hackscanner_lib;

mod bench_helper;

mod walkdir {
    use super::*;
    use test::Bencher;
    use hackscanner_lib::file_finder::walkdir::FileFinder;

    #[bench]
    fn bench_find_files(b: &mut Bencher) {
        bench_helper::bench_find_files(&FileFinder::new(), b);
    }

    #[bench]
    fn bench_find_files_without_rules(b: &mut Bencher) {
        bench_helper::bench_find_files_without_rules(&FileFinder::new(), b);
    }
}
