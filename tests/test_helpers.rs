extern crate hackscanner_lib;

use hackscanner_lib::*;

pub fn contains_path<E: DirEntryTrait>(paths: &Vec<E>, test_path: String) -> bool {
    paths.into_iter()
        .find(|entry| {
            let path_as_string = entry.path().to_string_lossy().into_owned();

            path_as_string == test_path
        })
        .is_some()
}
