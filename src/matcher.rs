use rule::RuleTrait;
use rule::PatternRule;
use dir_entry::*;

pub struct Matcher {}

impl Matcher {
    /// Check if the entry's path matches the given rule
    pub fn match_entry_path<D: DirEntryTrait>(rule: &PatternRule, entry: &D) -> bool {
        let path_as_string: String = entry.path().to_string_lossy().into_owned();

        match rule.path() {
            Some(p) => p.is_match(&path_as_string),
            None => false,
        }
    }
}
