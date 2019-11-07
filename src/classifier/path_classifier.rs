use crate::rule::*;
use super::Classification;
use super::ClassifierTrait;
use super::Violation;
use crate::DirEntryTrait;
use crate::matcher::Matcher;

pub struct PathClassifier {}

impl<'a, D: DirEntryTrait> ClassifierTrait<D> for PathClassifier {
    fn new(_: &D) -> Self {
        PathClassifier {}
    }

    fn classify(&mut self, entry: &D, rule: &PatternRule) -> Classification {
        if !rule.has_path() {
            return Classification::NotApplicable;
        }
        if Matcher::match_entry_path(rule, entry) {
            trace!("Rule's path does match");

            Classification::Match(Violation::from(rule))
        } else {
            trace!("Rule's path does not match");

            Classification::NoMatch
        }
    }
}
