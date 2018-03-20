extern crate hackscanner_lib;

mod test_helpers;

mod file_finder_test_suite_ftw {
    use std::thread;
    use test_helpers::*;
    use hackscanner_lib::file_finder::FileFinderTrait;
    use hackscanner_lib::file_finder::ftw::FileFinder;

    #[test]
    fn find_files_test() {
        let rules = get_rules_multiple_results();
        let matches = FileFinder::find(&FileFinder::new(), env!("CARGO_MANIFEST_DIR"), &rules);
        assert_multiple_paths(matches);
    }

    #[test]
    fn find_files_one_test() {
        let rules = get_rules_single_result();

        // Call `find` multiple times to make sure the results are cleared between calls
        FileFinder::find(&FileFinder::new(), env!("CARGO_MANIFEST_DIR"), &rules);
        let matches = FileFinder::find(&FileFinder::new(), env!("CARGO_MANIFEST_DIR"), &rules);

        assert_single_path(matches);
    }

    #[test]
    fn find_files_one_multi_threading_test() {
        for _ in 0..4 {
            thread::spawn(|| {
                let rules = get_rules_single_result();
                let matches = FileFinder::find(&FileFinder::new(), env!("CARGO_MANIFEST_DIR"), &rules);
                assert_single_path(matches);
            });
        }
    }
}
