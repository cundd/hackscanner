use regex::Regex;
use errors::*;
use severity::Severity;
use super::RuleTrait;
use super::rule::Rule;

/// Rule with compiled regular expression members
#[derive(Debug)]
pub struct PatternRule {
    rule: Rule,
    path: Option<Regex>,
    content: Option<Regex>,
}

impl PatternRule {
    pub fn from_rule(rule: &Rule) -> Result<PatternRule, Error> {
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

    pub fn from_rules(rules: &Vec<Rule>) -> Vec<Result<PatternRule, Error>> {
        rules.into_iter().map(|rule| PatternRule::from_rule(rule)).collect()
    }

    pub fn from_rules_filtered(rules: &Vec<Rule>) -> Vec<PatternRule> {
        trace!("Will transform rules to PatternRules");
        let result = Self::from_rules(rules).into_iter()
            .filter_map(|result| result.ok())
            .collect();
        trace!("Did transform rules to PatternRules");

        result
    }

    fn build_regex(pattern: &str) -> Result<Regex, Error> {
        let regex = Regex::new(&format!("(?i){}", pattern))?;

        Ok(regex)
    }
}

impl RuleTrait<Regex> for PatternRule {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_rule_test() {
        let pattern_rule = PatternRule::from_rule(&Rule::new(Severity::NOTICE, None, None)).unwrap();
        assert!(pattern_rule.path().is_none());
        assert!(pattern_rule.content().is_none());
        assert_eq!(pattern_rule.severity(), Severity::NOTICE);

        let pattern_rule = PatternRule::from_rule(&Rule::new(Severity::EASE, None, None)).unwrap();
        assert!(pattern_rule.path().is_none());
        assert!(pattern_rule.content().is_none());
        assert_eq!(pattern_rule.severity(), Severity::EASE);

        let pattern_rule = PatternRule::from_rule(
            &Rule::new(
                Severity::EASE,
                Some("^\\d{4}-\\d{2}-\\d{2}$".to_owned()),
                None,
            )
        ).unwrap();
        assert!(pattern_rule.path().is_some());
        assert!(pattern_rule.path().unwrap().is_match("2014-01-01"));

        let pattern_rule = PatternRule::from_rule(
            &Rule::new(
                Severity::EASE,
                None,
                Some("^\\d{4}-\\d{2}-\\d{2}$".to_owned()),
            )
        ).unwrap();
        assert!(pattern_rule.content().is_some());
        assert!(pattern_rule.content().unwrap().is_match("2014-01-01"));
    }
}
