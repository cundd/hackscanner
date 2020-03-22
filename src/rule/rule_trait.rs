use crate::rule::RulePath;
use crate::Severity;

/// Generic trait for Rule functions
pub trait RuleTrait<T> {
    /// Return the name
    fn name(&self) -> &str;

    /// Return the path(-pattern)
    fn path(&self) -> RulePath;

    /// Return the content to check against
    fn content(&self) -> Option<T>;

    /// Return the severity
    fn severity(&self) -> Severity;

    /// Return if the Rule has a content to check against
    fn has_content(&self) -> bool {
        self.content().is_some()
    }

    /// Return if the `Rule`'s path is a regular expression
    fn is_regex_path(&self) -> bool {
        if let RulePath::Regex(_) = self.path() {
            true
        } else {
            false
        }
    }
}
