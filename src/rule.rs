use regex::Regex;
use errors::*;

/// Generic trait for Rule functions
pub trait RuleTrait<T> {
    fn path(&self) -> Option<T>;
    fn content(&self) -> Option<T>;
    fn score(&self) -> i8;
}

/// "raw" Rule
#[derive(Debug, Clone)]
pub struct Rule {
    path: Option<String>,
    content: Option<String>,
    score: i8,
}

impl Rule {
    pub fn new(path: Option<String>, content: Option<String>, score: i8) -> Rule
    {
        Rule {
            path,
            content,
            score,
        }
    }
}

impl RuleTrait<String> for Rule {
    fn path(&self) -> Option<String> {
        self.path.clone()
    }

    fn content(&self) -> Option<String> {
        self.content.clone()
    }
    fn score(&self) -> i8 {
        self.score
    }
}

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
            Some(p) => Some(Regex::new(&p)?)
        };
        let content = match rule.content() {
            None => None,
            Some(c) => Some(Regex::new(&c)?)
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
        Self::from_rules(rules).into_iter()
            .filter_map(|result| result.ok())
            .collect()
    }
}

impl RuleTrait<Regex> for PatternRule {
    fn path(&self) -> Option<Regex> {
        self.path.clone()
    }

    fn content(&self) -> Option<Regex> {
        self.content.clone()
    }

    fn score(&self) -> i8 {
        self.rule.score()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_rule_test() {
        let pattern_rule = PatternRule::from_rule(&Rule::new(None, None, 8)).unwrap();
        assert!(pattern_rule.path().is_none());
        assert!(pattern_rule.content().is_none());
        assert_eq!(pattern_rule.score(), 8);

        let pattern_rule = PatternRule::from_rule(&Rule::new(None, None, -8)).unwrap();
        assert!(pattern_rule.path().is_none());
        assert!(pattern_rule.content().is_none());
        assert_eq!(pattern_rule.score(), -8);

        let pattern_rule = PatternRule::from_rule(
            &Rule::new(
                Some("^\\d{4}-\\d{2}-\\d{2}$".to_owned()),
                None,
                -8,
            )
        ).unwrap();
        assert!(pattern_rule.path().is_some());
        assert!(pattern_rule.path().unwrap().is_match("2014-01-01"));

        let pattern_rule = PatternRule::from_rule(
            &Rule::new(
                None,
                Some("^\\d{4}-\\d{2}-\\d{2}$".to_owned()),
                -8,
            )
        ).unwrap();
        assert!(pattern_rule.content().is_some());
        assert!(pattern_rule.content().unwrap().is_match("2014-01-01"));
    }
}
