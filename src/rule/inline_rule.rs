use severity::Severity;
use super::RuleTrait;

/// "inline" on-the-fly Rule
#[derive(Debug, Clone, Deserialize)]
pub struct InlineRule {
    name: String,
    path: Option<String>,
    content: Option<String>,
    severity: Severity,
}

impl InlineRule {
    /// Build a new rule
    pub fn new(name: String, score: Severity, path: Option<String>, content: Option<String>) -> Self
    {
        InlineRule {
            name,
            path,
            content,
            severity: score,
        }
    }
}

impl RuleTrait<String> for InlineRule {
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
