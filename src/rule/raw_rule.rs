use super::RuleTrait;
use crate::severity::Severity;

/// "raw" Rule
#[derive(Debug, Clone, Deserialize, PartialOrd, PartialEq)]
pub struct RawRule {
    name: String,
    path: Option<String>,
    content: Option<String>,
    severity: Severity,
}

impl RawRule {
    /// Build a new rule
    pub fn new(
        name: String,
        score: Severity,
        path: Option<String>,
        content: Option<String>,
    ) -> Self {
        RawRule {
            name,
            path,
            content,
            severity: score,
        }
    }

    pub fn with_path<S1: Into<String>, S2: Into<String>>(
        name: S1,
        severity: Severity,
        path: S2,
    ) -> Self {
        Self::new(name.into(), severity, Some(path.into()), None)
    }

    pub fn with_content<S1: Into<String>, S2: Into<String>>(
        name: S1,
        severity: Severity,
        content: S2,
    ) -> Self {
        Self::new(name.into(), severity, None, Some(content.into()))
    }

    pub fn with_path_and_content<S1: Into<String>, S2: Into<String>, S3: Into<String>>(
        name: S1,
        severity: Severity,
        path: S2,
        content: S3,
    ) -> Self {
        Self::new(
            name.into(),
            severity,
            Some(path.into()),
            Some(content.into()),
        )
    }
}

impl RuleTrait<String> for RawRule {
    fn name(&self) -> &String {
        &self.name
    }
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
