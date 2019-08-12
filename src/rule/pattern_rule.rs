use std::convert::TryFrom;
use regex::Regex;
use errors::*;
use severity::Severity;
use super::RuleTrait;
use super::raw_rule::RawRule;
use Rule;

/// Rule with compiled regular expression members
#[derive(Debug, Clone)]
pub struct PatternRule {
    rule: RawRule,
    path: Option<Regex>,
    content: Option<Regex>,
}

impl PatternRule {
    pub fn from_rules_filtered(rules: &Vec<Rule>) -> Vec<PatternRule> {
        trace!("Will transform rules to PatternRules");
        let result = Self::from_rules(rules).into_iter()
            .filter_map(|result| result.ok())
            .collect();
        trace!("Did transform rules to PatternRules");

        result
    }

    pub fn from_rules(rules: &Vec<Rule>) -> Vec<Result<PatternRule>> {
        rules.into_iter().map(|rule| PatternRule::try_from(rule)).collect()
    }

    fn from_rule(rule: &Rule) -> Result<PatternRule> {
        match rule {
            Rule::RawRule(rule) => Self::from_raw_rule(rule),
            Rule::PatternRule(rule) => Ok(rule.clone()),
        }
    }

    fn from_raw_rule(rule: &RawRule) -> Result<PatternRule> {
        let path = match rule.path() {
            None => None,
            Some(pattern) => Some(Self::build_regex(&pattern)?)
        };
        let content = match rule.content() {
            None => None,
            Some(pattern) => Some(Self::build_regex(&pattern)?)
        };

        Ok(PatternRule {
            rule: rule.clone(),
            path,
            content,
        })
    }

    fn build_regex(pattern: &str) -> Result<Regex> {
        let regex = Regex::new(&format!("(?i){}", pattern))?;

        Ok(regex)
    }
}

impl RuleTrait<Regex> for PatternRule {
    fn name(&self) -> &String {
        self.rule.name()
    }

    fn path(&self) -> Option<Regex> {
        self.path.clone()
    }

    fn content(&self) -> Option<Regex> {
        self.content.clone()
    }

    fn severity(&self) -> Severity {
        self.rule.severity()
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
        let pattern_rule = PatternRule::from_rule(&Rule::new_raw("1", Severity::NOTICE, None, None)).unwrap();
        assert!(pattern_rule.path().is_none());
        assert!(pattern_rule.content().is_none());
        assert_eq!(pattern_rule.severity(), Severity::NOTICE);

        let pattern_rule = PatternRule::from_rule(&Rule::new_raw("2", Severity::EASE, None, None)).unwrap();
        assert!(pattern_rule.path().is_none());
        assert!(pattern_rule.content().is_none());
        assert_eq!(pattern_rule.severity(), Severity::EASE);

        let pattern_rule = PatternRule::from_rule(
            &Rule::new_raw(
                "3",
                Severity::EASE,
                Some("^\\d{4}-\\d{2}-\\d{2}$".to_owned()),
                None,
            )
        ).unwrap();
        assert!(pattern_rule.path().is_some());
        assert!(pattern_rule.path().unwrap().is_match("2014-01-01"));

        let pattern_rule = PatternRule::from_rule(
            &Rule::new_raw(
                "4",
                Severity::EASE,
                None,
                Some("^\\d{4}-\\d{2}-\\d{2}$".to_owned()),
            )
        ).unwrap();
        assert!(pattern_rule.content().is_some());
        assert!(pattern_rule.content().unwrap().is_match("2014-01-01"));
    }
}
