use crate::rule::RuleTrait;
use crate::rule::PatternRule;
use crate::dir_entry::*;

pub struct Matcher {}

impl Matcher {
    /// Check if the entry's path matches the given rule
    #[inline]
    pub fn match_entry_path<D: DirEntryTrait>(rule: &PatternRule, entry: &D) -> bool {
        let path_as_string: String = entry.path().to_string_lossy().into_owned();

        match rule.path() {
            Some(p) => p.is_match(&path_as_string),
            None => false,
        }
    }

    /// Check if the entry's content matches the given rule
    #[inline]
    pub fn match_entry_content(rule: &PatternRule, content: &str) -> bool {
        match rule.content() {
            None => false,
            Some(content_pattern) => content_pattern.is_match(content)
        }
    }
}
