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
extern crate core;
extern crate term;
extern crate clap;

pub mod errors;
mod fs;
mod dir_entry;
mod rule;
mod matcher;
mod severity;
mod join;
pub mod rating;
pub mod file_finder;
pub mod classifier;

pub use errors::*;
pub use dir_entry::*;
pub use rule::*;
pub use rating::*;
pub use file_finder::find_files;
pub use severity::Severity;
pub use join::join_violations;
