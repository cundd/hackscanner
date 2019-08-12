extern crate hackscanner_lib;
extern crate simplelog;

use hackscanner_lib::*;
use std::path::Path;
use simplelog::TerminalMode;

/// Assert that the `ratings` contain a Rating with the path matching `path` and a rating equal to or bigger than `score`
fn assert_contains_entry_with_score(ratings: &Vec<Rating>, score: isize, path: &str) {
    let mut matching_rating: Option<&Rating> = None;
    for rating in ratings {
        let path_as_string = rating.entry().path().to_string_lossy().into_owned();
        if path_as_string.contains(path) {
            matching_rating = Some(rating);
            if rating.rating() >= score {
                return;
            }
        }
    }

    if let Some(rating) = matching_rating {
        panic!(
            "Must find file: Found file {:?} did not fulfill score {} (rating is {})",
            rating.entry().path(),
            score,
            rating.rating()
        )
    }
    panic!("Must find file matching {:?}", path)
}

/// Assert that none of the Ratings contain a path matching `path` and a rating bigger than 0
fn assert_not_contains_entry(ratings: &Vec<Rating>, path: &str) {
    assert_not_contains_entry_with_score(ratings, 1, path)
}

/// Assert that none of the Ratings contain a path matching `path` and a rating equal to or bigger than `score`
fn assert_not_contains_entry_with_score(ratings: &Vec<Rating>, score: isize, path: &str) {
    for rating in ratings {
        let path_as_string = rating.entry().path().to_string_lossy().into_owned();
        if path_as_string.contains(path) {
            if rating.rating() >= score {
                panic!("Must not find entry {:?} with rating {}", rating.entry().path(), rating.rating());
            }
        }
    }
}

fn configure_logging(log_level_filter: simplelog::LevelFilter) {
    let mut loggers: Vec<Box<simplelog::SharedLogger>> = vec![];
    let mut config = simplelog::Config::default();
    config.time_format = Some("%H:%M:%S%.3f");

    if let Some(core_logger) = simplelog::TermLogger::new(log_level_filter, config, TerminalMode::Mixed) {
        loggers.push(core_logger);
    } else {
        loggers.push(simplelog::SimpleLogger::new(log_level_filter, config));
    }

    let _ = simplelog::CombinedLogger::init(loggers);
}

#[test]
fn run_rules_with_configuration_test() {
    configure_logging(simplelog::LevelFilter::Error);
    let configuration_file = format!(
        "{}{}",
        env!("CARGO_MANIFEST_DIR"),
        "/tests/resources/rules/rules.yaml"
    );
    let rules = &get_merged_rules(Some(&Path::new(&configuration_file))).unwrap();

    let files = file_finder::find_files(format!("{}/tests", env!("CARGO_MANIFEST_DIR")), rules);
    let pattern_rules = PatternRule::from_rules_filtered(rules);
    let ratings = rate_entries(&files, &pattern_rules);

    assert_not_contains_entry(&ratings, "/tests/resources/files/whitelist_me.php");
}

#[test]
fn run_builtin_rules_test() {
    configure_logging(simplelog::LevelFilter::Error);
    let rules = &get_merged_rules(None).unwrap();

    let files = file_finder::find_files(format!("{}/tests", env!("CARGO_MANIFEST_DIR")), rules);
    let pattern_rules = PatternRule::from_rules_filtered(rules);
    let ratings = rate_entries(&files, &pattern_rules);

    assert_contains_entry_with_score(&ratings, Severity::CRITICAL as isize, "/tests/resources/files/dezmond.php");
    assert_contains_entry_with_score(&ratings, Severity::MAJOR as isize, "/tests/resources/files/tx_mocfilemanager.php");
    assert_contains_entry_with_score(&ratings, Severity::MAJOR as isize, "/tests/resources/files/something.tx_mocfilemanager.php");
    assert_contains_entry_with_score(&ratings, Severity::NOTICE as isize, "/tests/resources/files/eval-in-file.php");
    assert_contains_entry_with_score(&ratings, Severity::MAJOR as isize, "tests/resources/files/multiple_violations.php");

    assert_contains_entry_with_score(&ratings, Severity::MINOR as isize, "tests/resources/files/typo3/fileadmin/user_upload/some_file.php");
    assert_contains_entry_with_score(&ratings, Severity::MAJOR as isize, "tests/resources/files/typo3/typo3conf/l10n/someext/some_file.php");
    assert_contains_entry_with_score(&ratings, Severity::MINOR as isize, "tests/resources/files/typo3/typo3temp/bad_file.php");
    assert_contains_entry_with_score(&ratings, Severity::MINOR as isize, "tests/resources/files/typo3/typo3temp/various_subdir/bad_file.php");
    assert_contains_entry_with_score(&ratings, Severity::MINOR as isize, "tests/resources/files/typo3/typo3temp/autoload-tests/bad_file.php");
    assert_contains_entry_with_score(&ratings, Severity::NOTICE as isize, "tests/resources/files/typo3/typo3conf/bad.php");
    assert_contains_entry_with_score(&ratings, Severity::MAJOR as isize, "tests/resources/files/typo3/typo3conf/ext/static_info_tables/Classes/static_info_tables.php");
    assert_not_contains_entry(&ratings, "tests/resources/files/typo3/typo3temp/Cache/allowed_file.php");
    assert_not_contains_entry(&ratings, "tests/resources/files/typo3/typo3temp/var/Cache/allowed_file.php");
    assert_not_contains_entry(&ratings, "tests/resources/files/typo3/typo3temp/autoload/autoload_allowed_file.php");
    assert_not_contains_entry(&ratings, "tests/resources/files/typo3/typo3temp/autoload-tests/autoload_allowed_file.php");
    assert_not_contains_entry(&ratings, "tests/resources/files/typo3/typo3temp/ExtensionManager/UpdateScripts/ext_update36596ab430661a78499d678a5bb65a9c.php");
    assert_not_contains_entry(&ratings, "tests/resources/files/typo3/typo3temp/var/transient/ext_updatebac283f6edfa19007d6b23122ff69aeb.php");
    assert_contains_entry_with_score(&ratings, Severity::MINOR as isize, "tests/resources/files/typo3/typo3temp/autoload/autoload_subfolder/bad_file.php");
}



