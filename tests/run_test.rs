extern crate hackscanner_lib;
extern crate simplelog;

use hackscanner_lib::*;

fn has_entry_with_score(ratings: &Vec<Rating>, min_severity: isize, path: &str) -> bool {
    let mut matching_rating: Option<&Rating> = None;
    for rating in ratings {
        let path_as_string = rating.entry().path().to_string_lossy().into_owned();
        if path_as_string.contains(path) {
            matching_rating = Some(rating);
            if rating.rating() >= min_severity {
                return true;
            }
        }
    }

    if let Some(rating) = matching_rating {
        panic!(
            "Found file {:?} did not fulfill min_severity {} (rating is {})",
            rating.entry().path(),
            min_severity,
            rating.rating()
        )
    }

    false
}

fn configure_logging(log_level_filter: simplelog::LevelFilter) {
    let mut loggers: Vec<Box<simplelog::SharedLogger>> = vec![];
    let mut config = simplelog::Config::default();
    config.time_format = Some("%H:%M:%S%.3f");

    if let Some(core_logger) = simplelog::TermLogger::new(log_level_filter, config) {
        loggers.push(core_logger);
    } else {
        loggers.push(simplelog::SimpleLogger::new(log_level_filter, config));
    }

    match simplelog::CombinedLogger::init(loggers) {
        Ok(_) => (),
        Err(e) => panic!(e),
    }
}

#[test]
fn run_builtin_rules_test() {
    configure_logging(simplelog::LevelFilter::Trace);
    let rules = &get_builtin_rules();

    let files = file_finder::find_files(format!("{}/tests", env!("CARGO_MANIFEST_DIR")), rules);
    let pattern_rules = PatternRule::from_rules_filtered(rules);
    let ratings = classifier::classify_entries(&files, &pattern_rules);

    assert!(has_entry_with_score(&ratings, Severity::CRITICAL as isize, "/tests/resources/files/dezmond.php"));
    assert!(has_entry_with_score(&ratings, Severity::MAJOR as isize, "/tests/resources/files/tx_mocfilemanager.php"));
    assert!(has_entry_with_score(&ratings, Severity::MAJOR as isize, "/tests/resources/files/something.tx_mocfilemanager.php"));
    assert!(has_entry_with_score(&ratings, Severity::NOTICE as isize, "/tests/resources/files/eval-in-file.php"));
    assert!(has_entry_with_score(&ratings, Severity::MAJOR as isize, "tests/resources/files/multiple_violations.php"));

    assert!(has_entry_with_score(&ratings, Severity::MINOR as isize, "tests/resources/files/typo3/fileadmin/user_upload/some_file.php"));
    assert!(has_entry_with_score(&ratings, Severity::MINOR as isize, "tests/resources/files/typo3/typo3conf/l10n/someext/some_file.php"));
}



