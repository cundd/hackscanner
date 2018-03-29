#![allow(unused)]
extern crate hackscanner_lib;

use std::thread;
use hackscanner_lib::*;
use hackscanner_lib::file_finder::FileFinderTrait;
use std::path::Path;

pub fn get_test_dir() -> String {
    format!("{}/tests", env!("CARGO_MANIFEST_DIR"))
}

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

pub fn test_multi_threading<D, F>(file_finder: F)
    where D: DirEntryTrait, F: FileFinderTrait<DirEntry=D> + 'static + ::std::marker::Send + Clone
{
    let mut threads = vec![];
    for _ in 0..4 {
        let file_finder = file_finder.clone();
        let handle = thread::spawn(move || {
            let rules = get_rules_single_result();
            let matches = file_finder.find(get_test_dir(), &rules);
            assert_single_path(matches);
        });

        threads.push(handle);
    }

    for thread in threads {
        let _ = thread.join();
    }
}

pub fn contains_path<E: DirEntryTrait>(paths: &Vec<E>, test_path: String) -> bool {
    paths.into_iter()
        .find(|entry| {
            let path_as_string = entry.path().to_string_lossy().into_owned();

            path_as_string == test_path
        })
        .is_some()
}

pub fn get_entry_for_path<E: DirEntryTrait>(paths: &Vec<E>, test_path: String) -> Option<&E> {
    paths.into_iter()
        .find(|entry| {
            let path_as_string = entry.path().to_string_lossy().into_owned();

            path_as_string == test_path
        })
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
