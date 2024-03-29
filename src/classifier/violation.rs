use crate::classifier::content_classifier::*;
use crate::errors::Error;
use crate::Rule;
use crate::{RuleTrait, Severity};
use std::error::Error as StdError;

#[derive(Debug, Clone)]
pub struct Violation {
    #[allow(unused)]
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

    pub fn with_rule_and_file_io_error(
        rule: Rule,
        error: &ContentClassificationError,
    ) -> Option<Self> {
        if error.kind() == ContentClassificationErrorKind::NotExists {
            return None;
        }
        let severity = Severity::NOTICE;
        let name = rule.name().to_owned();

        Some(Violation {
            rule: Some(rule),
            name,
            severity,
        })
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

impl From<&dyn StdError> for Violation {
    fn from(error: &dyn StdError) -> Self {
        Self::with_name_and_severity(error.to_string(), Severity::NOTICE)
    }
}

impl From<&Error> for Violation {
    fn from(error: &Error) -> Self {
        Self::with_name_and_severity(error.to_string(), Severity::NOTICE)
    }
}

impl From<&Rule> for Violation {
    fn from(rule: &Rule) -> Self {
        Self::with_rule(rule.clone())
    }
}
