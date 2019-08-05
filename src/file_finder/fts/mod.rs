extern crate core;

use std::ffi::CStr;
use std::ffi::CString;

use errors::*;
use super::FileFinderTrait;
use fs::StandaloneFileType;
use dir_entry::StandaloneDirEntry;
use std::path::Path;
use std::fmt::Debug;

/// Bindings generation
///
/// macOS:
///
/// ```bash
/// bindgen /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/fts.h \
///     -o src/file_finder/fts/bindings_macos.rs \
///     --whitelist-type FTSENT \
///     --whitelist-type FTS \
///     --whitelist-var 'FTS_.*' \
///     --whitelist-function fts_open \
///     --whitelist-function fts_children \
///     --whitelist-function fts_read \
///     --whitelist-function fts_close
/// ```
///
///
mod bindings;

use self::bindings::*;

//#[cfg(target_os = "macos")]
//mod bindings_macos;

//#[cfg(target_os = "macos")]
//use self::bindings_macos::*;
use std::path::PathBuf;

#[allow(unused)]
extern "C" fn compare(arg1: *const *const FTSENT, arg2: *const *const FTSENT)
                      -> ::std::os::raw::c_int {
    1
}

fn collect_dir_entries(root: &str) -> Result<Vec<StandaloneDirEntry>, Error> {
    let mut entries: Vec<StandaloneDirEntry> = vec![];

    unsafe {
        let root_c = match CString::new(root) {
            Ok(root_c) => root_c,
            Err(e) => bail!(ErrorKind::BindingError(format!("{}",e)))
        };

        let file_system = fts_open(&(root_c.as_bytes_with_nul().as_ptr() as *const i8), (FTS_COMFOLLOW | FTS_NOCHDIR) as i32, None);
        if file_system.is_null() {
            bail!(ErrorKind::BindingError("Result of fts_open() is NULL".to_owned()));
        }

        let mut child;
        let mut parent = fts_read(file_system);

        while !parent.is_null() {
            child = fts_children(file_system, 0);
            if !child.is_null() {
                loop {
                    if (*child).fts_info as u32 == FTS_F {
                        entries.push(dir_entry_from_fts_entry(child.as_ref().unwrap()));
                    }
                    if child.is_null() || (*child).fts_link.is_null() {
                        break;
                    }
                    child = (*child).fts_link;
                };
            }
            parent = fts_read(file_system);
        }
        fts_close(file_system);
    }

    Ok(entries)
}

#[inline]
fn dir_entry_from_fts_entry(entry: &FTSENT) -> StandaloneDirEntry {
    let path = unsafe { CStr::from_ptr(entry.fts_path).to_str().unwrap() };
    let name = unsafe { CStr::from_ptr(entry.fts_name.as_ptr()).to_str().unwrap() };

    let dir_entry = StandaloneDirEntry::from_path_with_file_type(
        PathBuf::from(format!("{}{}", path, name)),
        StandaloneFileType::File,
    );

    match dir_entry {
        Ok(dir_entry) => dir_entry,
        Err(_) => panic!("from_path_with_file_type() must return Ok"),
    }
}

#[derive(Clone)]
pub struct FileFinder {}

impl FileFinder {
    pub fn new() -> Self {
        FileFinder {}
    }
}

impl FileFinderTrait for FileFinder {
    type DirEntry = StandaloneDirEntry;
    fn walk_dir<P: AsRef<Path> + Debug + Clone, F>(&self, root: P, filter: F) -> Vec<Self::DirEntry>
        where F: FnMut(&Self::DirEntry) -> bool {
        let entries = collect_dir_entries(&root.as_ref().to_string_lossy().into_owned());

        match entries {
            Ok(entries) => entries.into_iter().filter(filter).collect(),
            Err(error) => {
                error!("{}", error);
                vec![]
            }
        }
    }
}

#[cfg(feature = "fts")]
#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn collect_dir_entries_test() {
        let r = collect_dir_entries(&format!("{}/tests", env!("CARGO_MANIFEST_DIR")));
        assert!(r.is_ok());

        let v = r.unwrap();
        assert!(25 < v.len(), "Expected result length to be bigger than 25, got {}", v.len());
    }

    #[test]
    fn walk_dir_test() {
        let r = FileFinder::walk_dir(&FileFinder::new(), &format!("{}/tests", env!("CARGO_MANIFEST_DIR")), |_| true);
        assert!(25 < r.len(), "Expected result length to be bigger than 25, got {}", r.len());
    }

    #[test]
    fn find_test() {
        let r = FileFinder::walk_dir(&FileFinder::new(), &format!("{}/tests", env!("CARGO_MANIFEST_DIR")), |_| true);
        assert!(25 < r.len(), "Expected result length to be bigger than 25, got {}", r.len());
    }
}
