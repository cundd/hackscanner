use std::path::Path;
use std::fmt::Debug;
use walkdir::WalkDir;

use rule::*;
use dir_entry::*;
use matcher::Matcher;

/// Return all [`DirEntry`s] that match at least one of the [`Rule`s] starting at `root`
pub fn find_files<P: AsRef<Path> + Debug + Clone>(root: P, rules: &Vec<Rule>) -> Vec<walkdir::DirEntry> {
    let pattern_rules = PatternRule::from_rules_filtered(rules);

    iterate_files(root, |entry: &walkdir::DirEntry| {
        pattern_rules.iter()
            .find(|rule| Matcher::match_entry_path(rule, entry))
            .is_some()
    })
}

fn iterate_files<P: AsRef<Path> + Debug + Clone, F>(root: P, callback: F) -> Vec<walkdir::DirEntry>
    where F: Fn(&walkdir::DirEntry) -> bool {
    trace!("Will search files in root {:?}", root);

    let result = WalkDir::new(root.clone())
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| walkdir::DirEntry::from_dir_entry(e))
        .filter(callback)
        .collect();
    trace!("Did search files in root {:?}", root);

    result
}
