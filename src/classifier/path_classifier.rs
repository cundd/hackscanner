use super::Classification;
use super::ClassifierTrait;
use super::Violation;
use crate::matcher::Matcher;
use crate::rule::*;
use crate::DirEntryTrait;

pub struct PathClassifier {}

impl<'a, D: DirEntryTrait> ClassifierTrait<D> for PathClassifier {
    fn new(_: &D) -> Self {
        PathClassifier {}
    }

    fn classify(&mut self, entry: &D, rule: &Rule) -> Classification {
        if Matcher::match_entry_path(rule, entry) {
            trace!("Rule's path does match");

            Classification::Match(Violation::from(rule))
        } else {
            trace!("Rule's path does not match");

            Classification::NoMatch
        }
    }
}
