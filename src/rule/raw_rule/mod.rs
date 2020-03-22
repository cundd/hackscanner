use crate::severity::Severity;
use crate::rule::rule_path::RulePath;
use super::raw_path::string_or_struct;
pub(crate) use super::raw_path::RawPath;

/// "raw" Rule
#[derive(Debug, Clone, Deserialize, PartialOrd, PartialEq)]
pub(crate) struct RawRule {
    name: String,
    #[serde(deserialize_with = "string_or_struct")]
    path: RawPath,
    content: Option<String>,
    severity: Severity,
}

impl RawRule {
    pub fn is_regex_path(&self) -> bool {
        self.path.is_regex()
    }
    pub fn path_ref(&self) -> &str {
        self.path.as_str()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    #[allow(unused)]
    pub fn path(&self) -> RulePath {
        RulePath::String(self.path.as_str().to_owned())
    }

    pub fn content(&self) -> Option<String> {
        self.content.clone()
    }

    pub fn severity(&self) -> Severity {
        self.severity
    }
}
