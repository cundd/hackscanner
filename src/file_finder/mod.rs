use std::fmt::Debug;
use std::path::Path;

#[cfg(feature = "fts")]
pub mod fts;
pub mod ftw;
pub mod walkdir;

use crate::dir_entry::*;
use crate::matcher::Matcher;
use crate::rule::*;
use crate::Severity;

pub trait FileFinderTrait {
    type DirEntry: DirEntryTrait;

    /// Return all [`DirEntry`s] that match at least one of the [`Rule`s] starting at `root`
    fn find<P: AsRef<Path> + Debug + Clone>(&self, root: P, rules: &[Rule]) -> Vec<Self::DirEntry> {
        self.walk_dir(root, |entry: &Self::DirEntry| {
            if entry.file_type().is_dir() {
                return false;
            }
            if entry.file_type().is_symlink() {
                return false;
            }
            let path_as_string = entry.path().to_string_lossy();

            let mut store_entry = false;
            for rule in rules {
                // Check if the `Rule`'s path matches the current entry
                if Matcher::match_path_str(rule, path_as_string.as_ref()) {
                    // If the `Rule`'s path matches and the `Rule` is a whitelist-rule exit the loop
                    // and ignore the entry
                    if rule.severity() == Severity::WHITELIST {
                        return false;
                    }
                    store_entry = true;
                }
            }

            store_entry
        })
    }

    /// Walk through all files and directories under `root` and filter results with `filter`
    fn walk_dir<P: AsRef<Path> + Debug + Clone, F>(
        &self,
        root: P,
        filter: F,
    ) -> Vec<Self::DirEntry>
    where
        F: Fn(&Self::DirEntry) -> bool;
}

pub fn find_files<P: AsRef<Path> + Debug + Clone>(root: P, rules: &[Rule]) -> Vec<WalkdirDirEntry> {
    self::walkdir::FileFinder::find(&self::walkdir::FileFinder::new(), root, rules)
}
