use std::path::Path;
use std::fmt::Debug;

pub mod ftw;
#[cfg(feature = "fts")]
pub mod fts;
pub mod walkdir;

use rule::*;
use dir_entry::*;
use matcher::Matcher;

pub trait FileFinderTrait {
    type DirEntry: DirEntryTrait;

    /// Return all [`DirEntry`s] that match at least one of the [`Rule`s] starting at `root`
    fn find<P: AsRef<Path> + Debug + Clone>(&self, root: P, rules: &Vec<Rule>) -> Vec<Self::DirEntry> {
        let pattern_rules = PatternRule::from_rules_filtered(rules);

        self.walk_dir(root, |entry: &Self::DirEntry| {
            if entry.file_type().is_dir() {
                return false;
            }

            pattern_rules.iter()
                .find(|rule| Matcher::match_entry_path(rule, entry))
                .is_some()
        })
    }

    /// Walk through all files and directories under `root` and filter results with `filter`
    fn walk_dir<P: AsRef<Path> + Debug + Clone, F>(&self, root: P, filter: F) -> Vec<Self::DirEntry>
        where F: Fn(&Self::DirEntry) -> bool;
}

pub fn find_files<P: AsRef<Path> + Debug + Clone>(root: P, rules: &Vec<Rule>) -> Vec<WalkdirDirEntry> {
    self::walkdir::FileFinder::find(&self::walkdir::FileFinder::new(), root, rules)
}
