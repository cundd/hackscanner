mod rule;
mod pattern_rule;
mod builtin;

use severity::Severity;
pub use self::rule::Rule;
pub use self::pattern_rule::PatternRule;
pub use self::builtin::get_builtin_rules;

/// Generic trait for Rule functions
pub trait RuleTrait<T> {
    fn id(&self) -> isize;
    fn path(&self) -> Option<T>;
    fn content(&self) -> Option<T>;
    fn severity(&self) -> Severity;
}
