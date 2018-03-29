// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

extern crate serde;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;
extern crate simplelog;
extern crate walkdir;
extern crate regex;

extern crate libc;

pub mod errors;
mod fs;
mod dir_entry;
mod rule;
mod rating;
mod matcher;
mod severity;
pub mod file_finder;
pub mod classifier;

pub use errors::*;
pub use dir_entry::*;
pub use rule::*;
pub use rating::*;
pub use file_finder::find_files;
pub use severity::Severity;

//pub fn find_and_classify<'a, P: AsRef<Path>>(root: P, rules: &Vec<Rule>) -> (Vec<rating::Rating<'a>>,Vec<dir_entry::walkdir::DirEntry>) {
//    let files = file_finder::find_files(root, rules);
//
//    let pattern_rules = PatternRule::from_rules_filtered(rules);
//
//    ( classifier::classify_entries(&files, &pattern_rules), files)
//}
