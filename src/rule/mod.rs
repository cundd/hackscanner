mod rule;
mod pattern_rule;

pub use self::rule::Rule;
pub use self::pattern_rule::PatternRule;

/// Generic trait for Rule functions
pub trait RuleTrait<T> {
    fn path(&self) -> Option<T>;
    fn content(&self) -> Option<T>;
    fn score(&self) -> i8;
}
