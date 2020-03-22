//! Rule
//!
//! A [`Rule`] defines a set of checks to perform on a file. The [`Rule`] contains a `name`
//! describing the [`Rule`] and the `severity` of a violation. The [`Rule`] also may contain a
//! `path` and/or a `content`.
//!
//! If `path` **and** `content` are defined, a file violates the [`Rule`] if the file path matches
//! the [`Rule`]'s `path` **and** the file content matches the [`Rule`]s `content`.
//!
//! If only `path` is defined, a matching file path violates the [`Rule`].
//!
//! If only `content` is defined, a matching file content violates the [`Rule`].
mod builtin;
mod rule_trait;
mod pattern_rule;
mod raw_rule;
mod raw_path;
mod reader;
mod rule_path;

use crate::errors::*;
use std::path::Path;
pub use self::raw_path::RawPath;
pub use self::rule_path::RulePath;
pub use self::builtin::get_builtin_rules;
pub use self::pattern_rule::PatternRule as Rule;
pub use rule_trait::RuleTrait;

/// Read the `Rule`s from the given path and merge them with the builtin rules
pub fn get_merged_rules<P: AsRef<Path>>(path: P) -> Result<Vec<Rule>, Error> {
    let path = path.as_ref();

    let mut collection = reader::Reader::read_rules_from_file(path)?;
    info!("Read {} custom rule(s) from '{}'", collection.len(), path.display());
    trace!("Custom rules: {:?}", collection);
    collection.append(&mut get_builtin_rules());

    Ok(collection)
}
