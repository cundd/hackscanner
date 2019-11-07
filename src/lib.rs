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

pub use crate::errors::*;
pub use crate::dir_entry::*;
pub use crate::rule::*;
pub use crate::rating::*;
pub use crate::file_finder::find_files;
pub use crate::severity::Severity;
pub use crate::join::join_violations;
