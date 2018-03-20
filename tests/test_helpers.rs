extern crate hackscanner_lib;

use hackscanner_lib::*;

pub fn get_rules_multiple_results() -> Vec<Rule> {
    vec![
        Rule::new("1".to_string(), Severity::NOTICE, Some("tx_mocfilemanager".to_owned()), None)
    ]
}

pub fn get_rules_single_result() -> Vec<Rule> {
    vec![
        Rule::new("2".to_string(), Severity::NOTICE, Some("\\.tx_mocfilemanager".to_owned()), None)
    ]
}

pub fn assert_multiple_paths<D: DirEntryTrait>(matches: Vec<D>) {
//    assert_eq!(3, matches.len());

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

pub fn assert_single_path<D: DirEntryTrait>(matches: Vec<D>) {
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

pub fn contains_path<E: DirEntryTrait>(paths: &Vec<E>, test_path: String) -> bool {
    paths.into_iter()
        .find(|entry| {
            let path_as_string = entry.path().to_string_lossy().into_owned();

            path_as_string == test_path
        })
        .is_some()
}

//pub fn contains_path_ref<E: DirEntryTrait + ::std::marker::Sized>(paths: &Vec<&E>, test_path: String) -> bool {
//    paths.into_iter()
//        .find(|entry| {
//            let path_as_string = entry.path().to_string_lossy().into_owned();
//
//            path_as_string == test_path
//        })
//        .is_some()
//}
