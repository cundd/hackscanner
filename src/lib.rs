// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate walkdir;
extern crate regex;

pub mod errors;
mod dir_entry;
mod rule;
mod rating;
mod matcher;
pub mod file_finder;
pub mod classifier;

pub use errors::*;
pub use dir_entry::*;
pub use rule::*;
pub use file_finder::find_files;
