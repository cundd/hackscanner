pub mod test_helpers;

// FTS support is disable by default
#[cfg(feature = "fts")]
mod file_finder_test_suite_fts {
    use crate::test_helpers::*;
    use hackscanner_lib::file_finder::FileFinderTrait;
    use hackscanner_lib::file_finder::fts::FileFinder;

    #[test]
    fn find_files_test() {
        let rules = get_rules_multiple_results();
        let matches = FileFinder::new().find(get_test_dir(), &rules);
        assert_multiple_paths(matches);
    }

    #[test]
    fn find_files_one_test() {
        let rules = get_rules_single_result();

        // Call `find` multiple times to make sure the results are cleared between calls
        FileFinder::new().find(get_test_dir(), &rules);
        let matches = FileFinder::new().find(get_test_dir(), &rules);

        assert_single_path(matches);
    }

    #[test]
    fn find_files_one_multi_threading_test() {
        test_multi_threading(FileFinder::new());
    }
}
