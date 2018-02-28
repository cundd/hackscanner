//extern crate walkdir;
extern crate hackscanner_lib;

use hackscanner_lib::*;

fn contains_path(paths: &Vec<WalkdirDirEntry>, test_path: String) -> bool {
    paths.into_iter()
        .find(|entry| {
            let path_as_string = entry.path().to_string_lossy().into_owned();

            path_as_string == test_path
        })
        .is_some()
}

#[test]
fn find_files_test() {
    let rules = vec![Rule::new(1,Severity::NOTICE, Some("tx_mocfilemanager".to_owned()), None)];
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
    let rules = vec![Rule::new(2,Severity::NOTICE, Some("\\.tx_mocfilemanager".to_owned()), None)];
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
