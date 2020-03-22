use self::regex_cache::RegexCache;
use super::raw_rule::RawPath;
use super::raw_rule::RawRule;
use super::RuleTrait;
use crate::errors::*;
use crate::rule::rule_path::RulePath;
use crate::severity::Severity;
use regex::Regex;
use std::convert::TryFrom;

mod regex_cache;

/// Rule with compiled regular expression members
#[derive(Debug, Clone)]
pub struct PatternRule {
    name: String,
    path: String,
    is_regex_path: bool,
    path_regex: Option<Regex>,
    content: Option<Regex>,
    severity: Severity,
}

impl PartialEq for PatternRule {
    fn eq(&self, other: &Self) -> bool {
        if self.name != other.name {
            return false;
        }
        if self.path != other.path {
            return false;
        }
        if self.is_regex_path != other.is_regex_path {
            return false;
        }
        if self.severity != other.severity {
            return false;
        }
        if self.path_regex.is_some() != other.path_regex.is_some() {
            return false;
        }
        if self.content.is_some() != other.content.is_some() {
            return false;
        }
        if let Some(ref p) = self.path_regex {
            if p.as_str() != other.path_regex.as_ref().unwrap().as_str() {
                return false;
            }
        }
        if let Some(ref p) = self.content {
            if p.as_str() != other.content.as_ref().unwrap().as_str() {
                return false;
            }
        }

        true
    }
}

impl PatternRule {
    pub(crate) fn from_raw_rules(raw: Vec<RawRule>) -> Result<Vec<PatternRule>> {
        let mut container: Vec<PatternRule> = Vec::with_capacity(raw.len());
        trace!("Will transform rules to PatternRules");
        for r in raw {
            container.push(PatternRule::try_from(r)?);
        }
        trace!("Did transform rules to PatternRules");
        Ok(container)
    }

    pub fn new<S: Into<String>>(
        name: S,
        severity: Severity,
        raw_path: RawPath,
        content: Option<String>,
    ) -> Result<Self> {
        let is_regex_path = raw_path.is_regex();
        let path_regex = if is_regex_path {
            Some(Self::build_regex(raw_path.as_str())?)
        } else {
            None
        };
        let content = match content {
            Some(c) => Some(Self::build_regex(&c)?),
            None => None,
        };
        Ok(Self {
            name: name.into(),
            path: raw_path.as_str().to_owned(),
            is_regex_path,
            path_regex,
            content,
            severity,
        })
    }

    pub fn with_path<S1: Into<String>, S2: Into<RawPath>>(
        name: S1,
        severity: Severity,
        raw_path: S2,
    ) -> Result<Self> {
        let raw_path = raw_path.into();
        let is_regex_path = raw_path.is_regex();
        let path_regex = if is_regex_path {
            Some(Self::build_regex(raw_path.as_str())?)
        } else {
            None
        };
        Ok(Self {
            name: name.into(),
            path: raw_path.as_str().to_owned(),
            is_regex_path,
            path_regex,
            content: None,
            severity,
        })
    }

    pub fn with_path_and_content<S1: Into<String>, P: Into<RawPath>, S2: AsRef<str>>(
        name: S1,
        severity: Severity,
        raw_path: P,
        content: S2,
    ) -> Result<Self> {
        let raw_path = raw_path.into();
        let is_regex_path = raw_path.is_regex();
        let path_regex = if is_regex_path {
            Some(Self::build_regex(raw_path.as_str())?)
        } else {
            None
        };
        Ok(Self {
            name: name.into(),
            path: raw_path.as_str().to_owned(),
            is_regex_path,
            path_regex,
            content: Some(Self::build_regex(content.as_ref())?),
            severity,
        })
    }

    fn from_raw_rule(rule: &RawRule) -> Result<PatternRule> {
        let content = match rule.content() {
            None => None,
            Some(pattern) => Some(Self::build_regex(&pattern)?),
        };

        if rule.is_regex_path() {
            Ok(PatternRule {
                name: rule.name().to_owned(),
                path: rule.path_ref().to_owned(),
                is_regex_path: true,
                path_regex: Some(Self::build_regex(rule.path_ref())?),
                content,
                severity: rule.severity(),
                // rule: rule.to_owned(),
            })
        } else {
            Ok(PatternRule {
                name: rule.name().to_owned(),
                path: rule.path_ref().to_owned(),
                is_regex_path: false,
                path_regex: None,
                content,
                severity: rule.severity(),
                // rule: rule.to_owned(),
            })
        }
    }

    fn build_regex(pattern: &str) -> Result<Regex> {
        RegexCache::build(pattern)
    }
}

// fn is_pattern(pattern: &str) -> bool {
//     let chars = [
//         "^",
//         "*",
//         "$",
//         "?",
//         "+",
//         "[",
//         "\\.",
//     ];
//     for char in &chars {
//         if pattern.contains(*char) {
//             return true;
//         }
//     }
//     false
// }

impl RuleTrait<Regex> for PatternRule {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn path(&self) -> RulePath {
        match &self.path_regex {
            Some(p) => RulePath::Regex(p.clone()),
            None => RulePath::String(self.path.clone()),
        }
    }

    fn content(&self) -> Option<Regex> {
        self.content.clone()
    }

    fn severity(&self) -> Severity {
        self.severity
    }

    /// Return if the `Rule`'s path is a regular expression
    fn is_regex_path(&self) -> bool {
        self.is_regex_path
    }
}

impl TryFrom<RawRule> for PatternRule {
    type Error = Error;

    fn try_from(value: RawRule) -> Result<Self, Self::Error> {
        PatternRule::from_raw_rule(&value)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rule::raw_rule::RawPath;

    #[test]
    fn from_rule_test() {
        let pattern_rule =
            &PatternRule::new("1", Severity::NOTICE, RawPath::with_path("a-path"), None).unwrap();
        assert!(!pattern_rule.is_regex_path());
        assert!(pattern_rule.content().is_none());
        assert_eq!(pattern_rule.severity(), Severity::NOTICE);

        let pattern_rule =
            &PatternRule::new("2", Severity::EASE, RawPath::with_regex("a-path"), None).unwrap();
        assert_eq!(&pattern_rule.path().to_string(), "(?i)a-path");
        assert!(pattern_rule.is_regex_path());
        assert!(pattern_rule.content().is_none());
        assert_eq!(pattern_rule.severity(), Severity::EASE);

        let pattern_rule = &PatternRule::new(
            "3",
            Severity::EASE,
            RawPath::with_regex("^\\d{4}-\\d{2}-\\d{2}$"),
            None,
        )
        .unwrap();
        assert!(pattern_rule.path().regex().is_match("2014-01-01"));

        let pattern_rule = &PatternRule::new(
            "4",
            Severity::EASE,
            RawPath::with_path("a-path"),
            Some("^\\d{4}-\\d{2}-\\d{2}$".to_owned()),
        )
        .unwrap();
        assert!(pattern_rule.content().is_some());
        assert!(pattern_rule.content().unwrap().is_match("2014-01-01"));
    }
}
