use severity::Severity;
use super::RuleTrait;

/// "raw" Rule
#[derive(Debug, Clone, Deserialize)]
pub struct Rule {
    name: String,
    path: Option<String>,
    content: Option<String>,
    severity: Severity,
}

impl Rule {
    /// Build a new rule
    pub fn new(name: String, score: Severity, path: Option<String>, content: Option<String>) -> Rule
    {
        Rule {
            name,
            path,
            content,
            severity: score,
        }
    }

    pub fn with_path<S1: Into<String>, S2: Into<String>>(name: S1, severity: Severity, path: S2) -> Rule {
        Self::new(name.into(), severity, Some(path.into()), None)
    }

    pub fn with_content<S1: Into<String>, S2: Into<String>>(name: S1, severity: Severity, content: S2) -> Rule {
        Self::new(name.into(), severity, None, Some(content.into()))
    }

    pub fn with_path_and_content<S1: Into<String>, S2: Into<String>, S3: Into<String>>(name: S1, severity: Severity, path: S2, content: S3) -> Rule {
        Self::new(name.into(), severity, Some(path.into()), Some(content.into()))
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
    fn name(&self) -> &String {
        &self.name
    }
}
