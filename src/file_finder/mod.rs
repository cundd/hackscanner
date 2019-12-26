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
    fn find<P: AsRef<Path> + Debug + Clone>(
        &self,
        root: P,
        rules: &Vec<Rule>,
    ) -> Vec<Self::DirEntry> {
        let pattern_rules = PatternRule::from_rules_filtered(rules);

        self.walk_dir(root, |entry: &Self::DirEntry| {
            if entry.file_type().is_dir() {
                return false;
            }

            let mut store_entry = false;
            for rule in &pattern_rules {
                // Check if the `Rule` has a path that matches the current entry
                if rule.has_path() && Matcher::match_entry_path(&rule, entry) {
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

pub fn find_files<P: AsRef<Path> + Debug + Clone>(
    root: P,
    rules: &Vec<Rule>,
) -> Vec<WalkdirDirEntry> {
    self::walkdir::FileFinder::find(&self::walkdir::FileFinder::new(), root, rules)
}
