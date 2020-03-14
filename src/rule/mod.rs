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
mod pattern_rule;
mod raw_rule;
mod reader;
mod rule_path;

use self::builtin::get_builtin_rules;
pub use self::pattern_rule::PatternRule;
pub use self::raw_rule::RawRule;
use crate::errors::*;
use crate::severity::Severity;
pub use rule_path::RulePath;
use std::path::Path;

/// Generic trait for Rule functions
pub trait RuleTrait<T> {
    /// Return the name
    fn name(&self) -> &str;

    /// Return the path(-pattern)
    fn path(&self) -> RulePath;

    /// Return the content to check against
    fn content(&self) -> Option<T>;

    /// Return the severity
    fn severity(&self) -> Severity;

    /// Return if the Rule has a content to check against
    fn has_content(&self) -> bool {
        self.content().is_some()
    }

    /// Return if the `Rule`'s path is a regular expression
    fn is_regex_path(&self) -> bool {
        if let RulePath::Regex(_) = self.path() {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone)]
pub enum Rule {
    RawRule(RawRule),
    PatternRule(PatternRule),
}

impl Rule {
    pub fn name(&self) -> &str {
        match self {
            &Rule::RawRule(ref rule) => rule.name(),
            &Rule::PatternRule(ref rule) => rule.name(),
        }
    }

    pub fn severity(&self) -> Severity {
        match self {
            &Rule::RawRule(ref rule) => rule.severity(),
            &Rule::PatternRule(ref rule) => rule.severity(),
        }
    }

    pub fn path(&self) -> RulePath {
        match self {
            &Rule::RawRule(ref rule) => rule.path(),
            &Rule::PatternRule(ref rule) => rule.path(),
        }
    }

    pub fn content(&self) -> Option<RulePath> {
        match self {
            &Rule::RawRule(ref rule) => rule.content().map(|v| RulePath::String(v)),
            &Rule::PatternRule(ref rule) => rule.content().map(|v| RulePath::Regex(v)),
        }
    }

    /// Build new raw rules
    pub fn new_raw<S1: Into<String>, S2: Into<String>>(
        name: S1,
        score: Severity,
        path: S2,
        is_pattern: bool,
        content: Option<String>,
    ) -> Self {
        Rule::RawRule(RawRule::new(name.into(), score, path.into(), is_pattern, content))
    }

    pub fn raw_with_path<S1: Into<String>, S2: Into<String>>(
        name: S1,
        severity: Severity,
        path: S2,
        is_pattern: bool,
    ) -> Self {
        Rule::RawRule(RawRule::with_path(name, severity, path, is_pattern))
    }

    pub fn raw_with_path_and_content<S1: Into<String>, S2: Into<String>, S3: Into<String>>(
        name: S1,
        severity: Severity,
        path: S2,
        is_pattern: bool,
        content: S3,
    ) -> Self {
        Rule::RawRule(RawRule::with_path_and_content(
            name, severity, path, is_pattern, content,
        ))
    }
}

impl RuleTrait<RulePath> for Rule {
    fn name(&self) -> &str {
        Rule::name(self)
    }

    fn path(&self) -> RulePath {
        Rule::path(self)
    }

    fn content(&self) -> Option<RulePath> {
        Rule::content(self)
    }

    fn severity(&self) -> Severity {
        Rule::severity(self)
    }
}

pub fn get_merged_rules(path: Option<&Path>) -> Result<Vec<Rule>, Error> {
    match path {
        Some(p) => {
            let mut collection = reader::Reader::read_rules_from_file(p)?;
            info!("Read {} custom rule(s) from '{}'", collection.len(), p.display());
            trace!("Custom rules: {:?}", collection);
            collection.append(&mut get_builtin_rules());

            Ok(collection)
        }
        None => Ok(get_builtin_rules()),
    }
}

impl From<PatternRule> for Rule {
    fn from(pattern_rule: PatternRule) -> Self {
        Rule::PatternRule(pattern_rule)
    }
}

impl From<&PatternRule> for Rule {
    fn from(pattern_rule: &PatternRule) -> Self {
        Rule::PatternRule(pattern_rule.clone())
    }
}
