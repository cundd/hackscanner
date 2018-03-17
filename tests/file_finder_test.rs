extern crate hackscanner_lib;

mod test_helpers;

use hackscanner_lib::*;
use test_helpers::contains_path;

#[test]
fn find_files_test() {
    let rules = vec![Rule::new("1".to_string(), Severity::NOTICE, Some("tx_mocfilemanager".to_owned()), None)];
    let matches = file_finder::find_files(env!("CARGO_MANIFEST_DIR"), &rules);

    assert_eq!(3, matches.len());

    assert!(contains_path(
        &matches,
        format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/tests/resources/files/something.tx_mocfilemanager.php"
        ),
    ));
    assert!(contains_path(
        &matches,
        format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/tests/resources/files/tx_mocfilemanager.php"
        ),
    ));
    assert!(contains_path(
        &matches,
        format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/tests/resources/files/tx_mocfilemanager-test.sh"
        ),
    ));
}

#[test]
fn find_files_one_test() {
    let rules = vec![Rule::new("2".to_string(), Severity::NOTICE, Some("\\.tx_mocfilemanager".to_owned()), None)];

    // Call `find` multiple times to make sure the results are cleared between calls
    file_finder::find_files(env!("CARGO_MANIFEST_DIR"), &rules);
    let matches = file_finder::find_files(env!("CARGO_MANIFEST_DIR"), &rules);

    assert_eq!(1, matches.len());

    assert!(contains_path(
        &matches,
        format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/tests/resources/files/something.tx_mocfilemanager.php"
        ),
    ));
    assert_eq!(false, contains_path(
        &matches,
        format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/tests/resources/files/tx_mocfilemanager.php"
        ),
    ));
    assert_eq!(false, contains_path(
        &matches,
        format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/tests/resources/files/tx_mocfilemanager-test.sh"
        ),
    ));
}
