use severity::Severity;
use super::RuleTrait;
/// "raw" Rule
#[derive(Debug, Clone)]
pub struct Rule {
    path: Option<String>,
    content: Option<String>,
    severity: Severity,
}

impl Rule {
    /// Build a new rule
    pub fn new(score: Severity, path: Option<String>, content: Option<String>) -> Rule
    {
        Rule {
            path,
            content,
            severity: score,
        }
    }

    pub fn with_path<S: Into<String>>(severity: Severity, path: S) -> Rule {
        Self::new(severity, Some(path.into()), None)
    }

    pub fn with_content<S: Into<String>>(severity: Severity, content: S) -> Rule {
        Self::new(severity, None, Some(content.into()))
    }

    pub fn with_path_and_content<S1: Into<String>, S2: Into<String>>(severity: Severity, path: S1, content: S2) -> Rule {
        Self::new(severity, Some(path.into()), Some(content.into()))
    }
}

impl RuleTrait<String> for Rule {
    fn path(&self) -> Option<String> {
        self.path.clone()
    }

    fn content(&self) -> Option<String> {
        self.content.clone()
    }
    fn severity(&self) -> Severity {
        self.severity
    }
}
