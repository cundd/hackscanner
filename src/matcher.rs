use crate::dir_entry::*;
use crate::rule::PatternRule;
use crate::rule::RuleTrait;

pub struct Matcher {}

impl Matcher {
    /// Check if the entry's path matches the given rule
    pub fn match_entry_path<D: DirEntryTrait>(rule: &PatternRule, entry: &D) -> bool {
        let path_as_string = entry.path().to_string_lossy();

        Matcher::match_path_str(rule, &path_as_string)
    }

    /// Check if the given path matches the given rule
    pub fn match_path_str(rule: &PatternRule, path_as_string: &str) -> bool {
        match rule.path() {
            Some(p) => {
                trace!("Match rule '{}' with path '{}' against path '{}'", rule.name(), p, path_as_string);
                p.is_match(&path_as_string)
            }
            None => {
                info!("Rules without a path should be avoided for performance reason (will trigger a warning in next minor-release)");
                false
            }
        }
    }

    /// Check if the entry's content matches the given rule
    pub fn match_entry_content(rule: &PatternRule, content: &str) -> bool {
        match rule.content() {
            Some(content_pattern) => {
                trace!("Match rule '{}' with pattern '{}'", rule.name(), content_pattern);
                content_pattern.is_match(content)
            }
            None => false,
        }
    }
}
