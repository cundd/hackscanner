mod raw_rule;
mod pattern_rule;
mod inline_rule;
mod builtin;
mod reader;

use std::path::Path;
use errors::*;
use severity::Severity;
pub use self::raw_rule::RawRule;
pub use self::pattern_rule::PatternRule;
pub use self::inline_rule::InlineRule;
use self::builtin::get_builtin_rules;

/// Generic trait for Rule functions
pub trait RuleTrait<T> {
    fn name(&self) -> &String;
    fn path(&self) -> Option<T>;
    fn content(&self) -> Option<T>;
    fn severity(&self) -> Severity;
    fn has_content(&self) -> bool {
        self.content().is_some()
    }
}

#[derive(Debug, Clone)]
pub enum Rule {
    RawRule(RawRule),
    PatternRule(PatternRule),
    InlineRule(InlineRule),
}

impl Rule {
    pub fn from_pattern_rule(pattern_rule: &PatternRule) -> Self {
        Rule::PatternRule(pattern_rule.clone())
    }

    pub fn name(&self) -> &String {
        match self {
            &Rule::RawRule(ref rule) => rule.name(),
            &Rule::PatternRule(ref rule) => rule.name(),
            &Rule::InlineRule(ref rule) => rule.name(),
        }
    }

    pub fn severity(&self) -> Severity {
        match self {
            &Rule::RawRule(ref rule) => rule.severity(),
            &Rule::PatternRule(ref rule) => rule.severity(),
            &Rule::InlineRule(ref rule) => rule.severity(),
        }
    }

    pub fn has_content(&self) -> bool {
        match self {
            &Rule::RawRule(ref rule) => rule.has_content(),
            &Rule::PatternRule(ref rule) => rule.has_content(),
            &Rule::InlineRule(ref rule) => rule.has_content(),
        }
    }

    /// Build new inline rules
    pub fn new_inline<S1: Into<String>, S2: Into<String>, S3: Into<String>>(name: S1, path: S2, content: S3) -> Self {
        Rule::InlineRule(InlineRule::new(name.into(), Severity::NOTICE, Some(path.into()), Some(content.into())))
    }

    /// Build new raw rules
    pub fn new_raw(name: String, score: Severity, path: Option<String>, content: Option<String>) -> Self {
        Rule::RawRule(RawRule::new(name, score, path, content))
    }

    pub fn raw_with_path<S1: Into<String>, S2: Into<String>>(name: S1, severity: Severity, path: S2) -> Self {
        Rule::RawRule(RawRule::with_path(name, severity, path))
    }

    pub fn raw_with_content<S1: Into<String>, S2: Into<String>>(name: S1, severity: Severity, content: S2) -> Self {
        Rule::RawRule(RawRule::with_content(name, severity, content))
    }

    pub fn raw_with_path_and_content<S1: Into<String>, S2: Into<String>, S3: Into<String>>(name: S1, severity: Severity, path: S2, content: S3) -> Self {
        Rule::RawRule(RawRule::with_path_and_content(name, severity, path, content))
    }
}

impl<T> RuleTrait<T> for Rule {
    fn name(&self) -> &String {
        Rule::name(self)
    }

    fn path(&self) -> Option<T> {
        unimplemented!()
    }

    fn content(&self) -> Option<T> {
        unimplemented!()
    }

    fn severity(&self) -> Severity {
        Rule::severity(self)
    }

    fn has_content(&self) -> bool {
        Rule::has_content(self)
    }
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
