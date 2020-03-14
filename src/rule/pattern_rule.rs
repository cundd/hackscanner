use self::regex_cache::RegexCache;
use super::raw_rule::RawRule;
use super::RuleTrait;
use crate::errors::*;
use crate::severity::Severity;
use crate::Rule;
use regex::Regex;
use std::convert::TryFrom;
use crate::rule::rule_path::RulePath;

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

    // rule: RawRule,
}

impl PatternRule {
    pub fn from_rules_filtered(rules: &Vec<Rule>) -> Vec<PatternRule> {
        trace!("Will transform rules to PatternRules");
        let result = Self::from_rules(rules)
            .into_iter()
            .filter_map(|result| result.ok())
            .collect();
        trace!("Did transform rules to PatternRules");

        result
    }

    pub fn from_rules(rules: &Vec<Rule>) -> Vec<Result<PatternRule>> {
        rules
            .into_iter()
            .map(|rule| PatternRule::try_from(rule))
            .collect()
    }

    // pub fn path_str(&self) -> &str {
    //     self.rule.path_ref()
    // }

    fn from_rule(rule: &Rule) -> Result<PatternRule> {
        match rule {
            Rule::RawRule(rule) => Self::from_raw_rule(rule),
            Rule::PatternRule(rule) => Ok(rule.clone()),
        }
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

fn is_pattern(pattern: &str) -> bool {
    let chars = [
        "^",
        "*",
        "$",
        "?",
        "+",
        "[",
        "\\.",
    ];
    for char in &chars {
        if pattern.contains(*char) {
            return true;
        }
    }
    false
}

impl RuleTrait<Regex> for PatternRule {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn path(&self) -> RulePath {
        match &self.path_regex {
            Some(p) => RulePath::Regex(p.clone()),
            None => RulePath::String(self.path.clone())
        }
    }

    /// Return if the `Rule`'s path is a regular expression
    fn is_regex_path(&self) -> bool {
        self.is_regex_path
    }
    fn content(&self) -> Option<Regex> {
        self.content.clone()
    }

    fn severity(&self) -> Severity {
        self.severity
    }
}

impl TryFrom<&Rule> for PatternRule {
    type Error = Error;

    fn try_from(value: &Rule) -> Result<Self, Self::Error> {
        PatternRule::from_rule(value)
    }
}

impl TryFrom<Rule> for PatternRule {
    type Error = Error;

    fn try_from(value: Rule) -> Result<Self, Self::Error> {
        PatternRule::from_rule(&value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_rule_test() {
        let pattern_rule = PatternRule::from_rule(&Rule::new_raw(
            "1",
            Severity::NOTICE,
            "a-path",
            false,
            None,
        ))
            .unwrap();
        assert!(!pattern_rule.is_regex_path());
        assert!(pattern_rule.content().is_none());
        assert_eq!(pattern_rule.severity(), Severity::NOTICE);

        let pattern_rule =
            PatternRule::from_rule(&Rule::new_raw(
                "2",
                Severity::EASE,
                "a-path",
                true,
                None,
            ))
                .unwrap();
        assert_eq!(&pattern_rule.path().to_string(), "(?i)a-path");
        assert!(pattern_rule.is_regex_path());
        assert!(pattern_rule.content().is_none());
        assert_eq!(pattern_rule.severity(), Severity::EASE);

        let pattern_rule = PatternRule::from_rule(&Rule::new_raw(
            "3",
            Severity::EASE,
            "^\\d{4}-\\d{2}-\\d{2}$",
            true,
            None,
        ))
            .unwrap();
        assert!(pattern_rule.path().regex().is_match("2014-01-01"));

        let pattern_rule = PatternRule::from_rule(&Rule::new_raw(
            "4",
            Severity::EASE,
            "a-path",
            false,
            Some("^\\d{4}-\\d{2}-\\d{2}$".to_owned()),
        ))
            .unwrap();
        assert!(pattern_rule.content().is_some());
        assert!(pattern_rule.content().unwrap().is_match("2014-01-01"));
    }
}
