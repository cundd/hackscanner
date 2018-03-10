//extern crate walkdir;
extern crate hackscanner_lib;

mod test_helpers;

use hackscanner_lib::*;
use test_helpers::contains_path;
use file_finder::*;

#[test]
fn walkdir_find_files_test() {
    let rules = vec![Rule::new("1".to_string(), Severity::NOTICE, Some("tx_mocfilemanager".to_owned()), None)];
    let matches = walkdir::FileFinder::find(env!("CARGO_MANIFEST_DIR"), &rules);

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
fn walkdir_find_files_one_test() {
    let rules = vec![Rule::new("2".to_string(), Severity::NOTICE, Some("\\.tx_mocfilemanager".to_owned()), None)];
    let matches = file_finder::walkdir::FileFinder::find(env!("CARGO_MANIFEST_DIR"), &rules);

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
