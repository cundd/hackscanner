use std::ffi::CStr;
use std::ffi::CString;
use libc::c_char;
use libc::c_int;
use libc::stat;

use std::path::Path;
use std::path::PathBuf;
use std::fmt::Debug;
use std::vec::Vec;
use std::cell::RefCell;

use fs::constants::*;

use super::FileFinderTrait;
use dir_entry::StandaloneDirEntry;
use fs::StandaloneFileType;

#[repr(C)]
struct FTW {
    base: c_int,
    level: c_int,
}

type FtwFn = extern fn(fpath: *const c_char, sb: *const stat, typeflag: c_int) -> c_int;
type NftwFn = extern fn(fpath: *const c_char, sb: *const stat, typeflag: c_int, ftwbuf: *const FTW) -> c_int;


/// Wrapper for [`nftw`](https://linux.die.net/man/3/nftw)
/// int nftw(
///          const char *dirpath,
///          int (*fn) (const char *fpath, const struct stat *sb, int typeflag, struct FTW *ftwbuf),
///          int nopenfd,
///          int flags
/// );
extern {
    fn nftw(dirpath: *const c_char, f: NftwFn, nopenfd: c_int, flags: c_int) -> c_int;
}


/// Wrapper for [`ftw`](https://linux.die.net/man/3/ftw)
/// int ftw(
///         const char *dirpath,
///         int (*fn) (const char *fpath, const struct stat *sb, int typeflag),
///         int nopenfd
/// );
extern {
    fn ftw(dirpath: *const c_char, f: FtwFn, nopenfd: c_int) -> c_int;
}

/// Thread local vector to hold the found paths
thread_local! {
    static FOUND_PATHS: RefCell<Vec<StandaloneDirEntry>> = RefCell::new(vec![]);
}

/// Callback for [`ftw`](https://linux.die.net/man/3/ftw)
extern fn ftw_collector(
    fpath: *const c_char,
    _sb: *const stat,
    typeflag: c_int,
) -> c_int {
    unsafe {
        let path_string = CStr::from_ptr(fpath);

        let dir_entry = StandaloneDirEntry::from_path_with_file_type(
            PathBuf::from(path_string.to_string_lossy().into_owned()),
            StandaloneFileType::from_ftw(typeflag),
        );

        if let Ok(dir_entry) = dir_entry {
            FOUND_PATHS.with(|p| {
                p.borrow_mut().push(dir_entry)
            });
        }
    }

    0
}

/// Callback for [`nftw`](https://linux.die.net/man/3/nftw)
extern fn nftw_collector(
    fpath: *const c_char,
    _sb: *const stat,
    typeflag: c_int,
    _ftwbuf: *const FTW,
) -> c_int {
    unsafe {
        let path_string = CStr::from_ptr(fpath);

        if typeflag == FTW_F {
            let dir_entry = StandaloneDirEntry::from_path_with_file_type(
                PathBuf::from(path_string.to_string_lossy().into_owned()),
                StandaloneFileType::from_ftw(typeflag),
            );

            if let Ok(dir_entry) = dir_entry {
                FOUND_PATHS.with(|p| {
                    p.borrow_mut().push(dir_entry)
                });
            }
        }
    }

    0
}

pub struct FileFinder {}

impl FileFinderTrait for FileFinder {
    type DirEntry = StandaloneDirEntry;
    fn walk_dir<P: AsRef<Path> + Debug + Clone, F>(root: P, filter: F) -> Vec<Self::DirEntry>
        where F: FnMut(&Self::DirEntry) -> bool {
        let entries = collect_dir_entries_nftw(&root.as_ref().to_string_lossy().into_owned());

        entries.into_iter().filter(filter).collect()
    }
}


fn clear_entries() {
    FOUND_PATHS.with(|p| {
        p.borrow_mut().clear()
    });
}

fn collect_dir_entries_ftw(root: &str) -> Vec<StandaloneDirEntry> {
    clear_entries();
    unsafe {
        ftw(CString::new(root).unwrap().as_ptr(), ftw_collector, 20);
    }

    FOUND_PATHS.with(|p| {
        (*p.borrow()).clone()
    })
}

fn collect_dir_entries_nftw(root: &str) -> Vec<StandaloneDirEntry> {
    clear_entries();
    unsafe {
        nftw(CString::new(root).unwrap().as_ptr(), nftw_collector, 20, FTW_PHYS);
    }

    FOUND_PATHS.with(|p| {
        (*p.borrow()).clone()
    })
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn collect_dir_entries_ftw_test() {
        let r = collect_dir_entries_ftw(env!("CARGO_MANIFEST_DIR"));
        assert!(1000 < r.len(), "Expected result length to be bigger than 1000, got {}", r.len());
    }

    #[test]
    fn collect_dir_entries_nftw_test() {
        let r = collect_dir_entries_nftw(env!("CARGO_MANIFEST_DIR"));
        assert!(1000 < r.len(), "Expected result length to be bigger than 1000, got {}", r.len());
    }

    #[test]
    fn walk_dir_test() {
        let r = FileFinder::walk_dir(env!("CARGO_MANIFEST_DIR"), |_| true);
        assert!(1000 < r.len(), "Expected result length to be bigger than 1000, got {}", r.len());
    }

    #[test]
    fn find_test() {
        let r = FileFinder::walk_dir(env!("CARGO_MANIFEST_DIR"), |_| true);
        assert!(1000 < r.len(), "Expected result length to be bigger than 1000, got {}", r.len());
    }
}
