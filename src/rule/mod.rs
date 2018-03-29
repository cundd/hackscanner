mod rule;
mod pattern_rule;
mod builtin;
mod reader;

use std::path::Path;
use errors::*;
use severity::Severity;
pub use self::rule::Rule;
pub use self::pattern_rule::PatternRule;
use self::builtin::get_builtin_rules;

/// Generic trait for Rule functions
pub trait RuleTrait<T> {
    fn name(&self) -> &String;
    fn path(&self) -> Option<T>;
    fn content(&self) -> Option<T>;
    fn severity(&self) -> Severity;
}

pub fn get_merged_rules(path: Option<&Path>) -> Result<Vec<Rule>, Error> {
    match path {
        Some(p) => {
            let mut collection = reader::Reader::read_rules_from_file(p)?;
            collection.append(&mut get_builtin_rules());

            Ok(collection)
        }
        None => Ok(get_builtin_rules())
    }
}
