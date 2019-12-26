// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

pub mod classifier;
mod dir_entry;
pub mod errors;
pub mod file_finder;
mod fs;
mod join;
mod matcher;
pub mod rating;
mod rule;
mod severity;

pub use crate::dir_entry::*;
pub use crate::errors::*;
pub use crate::file_finder::find_files;
pub use crate::join::join_violations;
pub use crate::rating::*;
pub use crate::rule::*;
pub use crate::severity::Severity;
