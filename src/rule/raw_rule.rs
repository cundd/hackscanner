use super::RuleTrait;
use crate::severity::Severity;
use crate::rule::rule_path::RulePath;
//
// #[derive(Debug, Clone, Deserialize, PartialOrd, PartialEq)]
// pub struct RawPath {
//     path: String,
//     #[serde(default=false^)]
//     is_regex: bool,
// }

/// "raw" Rule
#[derive(Debug, Clone, Deserialize, PartialOrd, PartialEq)]
pub struct RawRule {
    name: String,
    path: String,
    is_regex_path: bool,
    content: Option<String>,
    severity: Severity,
}

impl RawRule {
    /// Build a new rule
    pub fn new(
        name: String,
        score: Severity,
        path: String,
        is_regex_path: bool,
        content: Option<String>,
    ) -> Self {
        RawRule {
            name,
            path,
            is_regex_path,
            content,
            severity: score,
        }
    }

    pub fn with_path<S1: Into<String>, S2: Into<String>>(
        name: S1,
        severity: Severity,
        path: S2,
        is_pattern: bool,
    ) -> Self {
        Self::new(name.into(), severity, path.into(), is_pattern, None)
    }

    pub fn with_path_and_content<S1: Into<String>, S2: Into<String>, S3: Into<String>>(
        name: S1,
        severity: Severity,
        path: S2,
        is_pattern: bool,
        content: S3,
    ) -> Self {
        Self::new(
            name.into(),
            severity,
            path.into(),
            is_pattern,
            Some(content.into()),
        )
    }

    pub fn is_regex_path(&self) -> bool {
        self.is_regex_path
    }
    pub fn path_ref(&self) -> &str {
        self.path.as_str()
    }
}

impl RuleTrait<String> for RawRule {
    fn name(&self) -> &str {
        &self.name
    }
    fn path(&self) -> RulePath {
        RulePath::String(self.path.clone())
    }
    fn content(&self) -> Option<String> {
        self.content.clone()
    }
    fn severity(&self) -> Severity {
        self.severity
    }
}
