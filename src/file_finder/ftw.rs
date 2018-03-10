extern crate libc;

use std::ffi::CStr;
use std::ffi::CString;
use self::libc::c_char;
use self::libc::c_int;
use self::libc::stat;

use std::path::Path;
use std::path::PathBuf;
use std::fmt::Debug;
use std::sync::Mutex;
use std::vec::Vec;

use super::FileFinderTrait;
use dir_entry::StandaloneDirEntry;

lazy_static! {
    static ref FOUND_PATHS: Mutex<Vec<String>> = Mutex::new(vec![]);
}


type WalkFn = extern fn(fpath: *const c_char, sb: *const stat, typeflag: c_int) -> c_int;


/// Wrapper for `ftw`
/// int ftw(
///          const char *dirpath,
///          int (*fn) (const char *fpath, const struct stat *sb, int typeflag),
///          int nopenfd
/// );
extern {
    fn ftw(dirpath: *const c_char, f: WalkFn, nopenfd: c_int) -> c_int;
}


extern fn x(fpath: *const c_char, sb: *const stat, typeflag: c_int) -> c_int {
    unsafe {
        let path_string = CStr::from_ptr(fpath);
        match FOUND_PATHS.lock() {
            Ok(ref mut p) => p.push(path_string.to_string_lossy().into_owned()),
            Err(e) => panic!("FOUND_PATHS is None {}", e)
        }
    }

    0
}

pub struct FileFinder {}

impl FileFinderTrait for FileFinder {
    type DirEntry = StandaloneDirEntry;
    fn walk_dir<P: AsRef<Path> + Debug + Clone, F>(root: P, filter: F) -> Vec<Self::DirEntry>
        where F: FnMut(&Self::DirEntry) -> bool {
        let entries = collect_dir_entries(root);

        entries.into_iter().filter(filter).collect()
    }
}

fn clear_entries() {
    match FOUND_PATHS.lock() {
        Ok(ref mut p) => p.clear(),
        Err(e) => panic!("FOUND_PATHS is None: {}", e)
    }
}

fn collect_dir_entries<P: AsRef<Path> + Debug + Clone>(root: P) -> Vec<StandaloneDirEntry> {
    clear_entries();
    let path_as_string = root.as_ref().to_string_lossy().into_owned();
    unsafe {
        ftw(CString::new(path_as_string).unwrap().as_ptr(), x, 20);
    }

    match FOUND_PATHS.lock() {
        Ok(ref mut p) => p.iter().
            filter_map(|path| {
                StandaloneDirEntry::from_path(PathBuf::from(path)).ok()
            }).collect(),
        Err(e) => panic!("FOUND_PATHS is None: {}", e)
    }
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn walk_dir_test() {
        let r = FileFinder::walk_dir(env!("CARGO_MANIFEST_DIR"), |_| true);
        assert!(1000 < r.len());
    }

    #[test]
    fn find_test() {
        let r = FileFinder::walk_dir(env!("CARGO_MANIFEST_DIR"), |_| true);
        assert!(1000 < r.len());
    }
}
