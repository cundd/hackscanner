extern crate hackscanner_lib;

mod test_helpers;

mod file_finder_test_suite {
    use crate::test_helpers::*;
    use hackscanner_lib::*;

    #[test]
    fn find_files_test() {
        let rules = get_rules_multiple_results();
        let matches = file_finder::find_files(get_test_dir(), &rules);
        assert_multiple_paths(matches);
    }

    #[test]
    fn find_files_one_test() {
        let rules = get_rules_single_result();

        // Call `find` multiple times to make sure the results are cleared between calls
        file_finder::find_files(get_test_dir(), &rules);
        let matches = file_finder::find_files(get_test_dir(), &rules);

        assert_single_path(matches);
    }
}
