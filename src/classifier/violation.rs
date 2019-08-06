use ::{Rule, Severity};
use std::error::Error as StdError;
use PatternRule;

#[derive(Debug, Clone)]
pub struct Violation {
    rule: Option<Rule>,
    name: String,
    severity: Severity,
}

impl Violation {
    pub fn with_rule(rule: Rule) -> Self {
        let severity = rule.severity();
        let name = rule.name().to_owned();
        Violation {
            rule: Some(rule),
            name,
            severity,
        }
    }

    pub fn with_name_and_severity(name: String, severity: Severity) -> Self {
        Violation {
            rule: None,
            name,
            severity,
        }
    }

    /// Return the name describing the Violation (e.g. the name of the violated Rule)
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Return the severity of the Violation (e.g. the severity of the violated Rule)
    pub fn severity(&self) -> Severity {
        self.severity
    }
}

impl From<&StdError> for Violation {
    fn from(error: &StdError) -> Self {
        Self::with_name_and_severity(error.description().to_owned(), Severity::NOTICE)
    }
}

impl From<&::errors::Error> for Violation {
    fn from(error: &::errors::Error) -> Self {
        Self::with_name_and_severity(error.description().to_owned(), Severity::NOTICE)
    }
}

impl From<Rule> for Violation {
    fn from(rule: Rule) -> Self {
        Self::with_rule(rule)
    }
}

impl From<&PatternRule> for Violation {
    fn from(rule: &PatternRule) -> Self {
        Self::with_rule(Rule::from(rule))
    }
}
