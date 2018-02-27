extern crate hackscanner_lib;
extern crate simplelog;

use hackscanner_lib::*;

fn has_entry_with_score(ratings: &Vec<Rating>, min_severity: isize, path: &str) -> bool {
    for rating in ratings {
        let path_as_string = rating.entry().path().to_string_lossy().into_owned();
        if rating.rating() >= min_severity && path_as_string.contains(path) {
            return true;
        }
    }
    false
}

fn configure_logging(log_level_filter: simplelog::LevelFilter) {
    let mut loggers: Vec<Box<simplelog::SharedLogger>> = vec![];

    if let Some(core_logger) = simplelog::TermLogger::new(log_level_filter, simplelog::Config::default()) {
        loggers.push(core_logger);
    } else {
        loggers.push(simplelog::SimpleLogger::new(log_level_filter, simplelog::Config::default()));
    }

    match simplelog::CombinedLogger::init(loggers) {
        Ok(_) => (),
        Err(e) => panic!(e),
    }
}

#[test]
fn run_builtin_rules_test() {
    configure_logging(simplelog::LevelFilter::Debug);
    let rules = &get_builtin_rules();

    let files = file_finder::find_files(env!("CARGO_MANIFEST_DIR"), rules);
    let pattern_rules = PatternRule::from_rules_filtered(rules);
    let ratings = classifier::classify_entries(&files, &pattern_rules);

    assert!(has_entry_with_score(&ratings, Severity::CRITICAL as isize, "/tests/resources/files/dezmond.php"));
    assert!(has_entry_with_score(&ratings, Severity::MAJOR as isize, "/tests/resources/files/tx_mocfilemanager.php"));
    assert!(has_entry_with_score(&ratings, Severity::MAJOR as isize, "/tests/resources/files/something.tx_mocfilemanager.php"));
    assert!(has_entry_with_score(&ratings, Severity::NOTICE as isize, "/tests/resources/files/eval-in-file.php"));
    assert!(has_entry_with_score(&ratings, Severity::MAJOR as isize, "tests/resources/files/multiple_violations.php"));
}



