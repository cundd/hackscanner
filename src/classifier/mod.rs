mod classification;
mod content_classifier;
mod path_classifier;
mod violation;

use crate::dir_entry::*;
use crate::rule::*;

use self::classification::*;
pub use self::content_classifier::ContentClassificationError;
use self::content_classifier::ContentClassifier;
use self::path_classifier::PathClassifier;
pub use self::violation::Violation;

pub fn classify_entries<'a, 'b, D: DirEntryTrait>(
    entries: &'a Vec<D>,
    rules: &'a Vec<PatternRule>,
) -> Vec<Vec<Violation>> {
    debug!("Will classify entries");
    let result = entries
        .iter()
        .map(|entry| classify_entry(entry, rules))
        .collect();
    debug!("Did classify entries");

    result
}

pub fn classify_entry<'a, 'b, D: DirEntryTrait>(
    entry: &'a D,
    rules: &'a Vec<PatternRule>,
) -> Vec<Violation> {
    let mut path_classifier = path_classifier::PathClassifier::new(entry);
    let mut content_classifier = content_classifier::ContentClassifier::new(entry);
    rules.iter().filter_map(|rule|
        match classify_entry_with_rule(&mut path_classifier, &mut content_classifier, entry, rule) {
            Classification::Empty => None,
            Classification::NotApplicable => panic!("Classification::NotApplicable must not be returned from `get_classification()`"),
            Classification::NoMatch => None,
            Classification::Match(violation) => Some(violation),
            Classification::Error(violation) => Some(violation)
        }
    ).collect()
}

trait ClassifierTrait<D: DirEntryTrait> {
    fn new(entry: &D) -> Self;
    fn classify(&mut self, entry: &D, rule: &PatternRule) -> Classification;
}

#[allow(deprecated)]
fn classify_entry_with_rule<D: DirEntryTrait>(
    path_classifier: &mut PathClassifier,
    content_classifier: &mut ContentClassifier,
    entry: &D,
    rule: &PatternRule,
) -> Classification {
    if rule.has_content() {
        let path_classification = ClassifierTrait::classify(path_classifier, entry, rule);
        match path_classification {
            Classification::Match(_) => { /* Path does match. Now check the content */ }
            Classification::NoMatch => {
                /* Path does not match. No need to check the content */
                return Classification::NoMatch;
            }
            Classification::Error(_) => {
                panic!("Classification::Error is not implemented for `PathClassifier`")
            }
            _ => unreachable!("{:?} not possible", path_classification),
        }

        let content_classification = ClassifierTrait::classify(content_classifier, entry, rule);
        match content_classification {
            Classification::NotApplicable => {
                unreachable!("Classification::NotApplicable not possible")
            }
            Classification::Empty => panic_empty(),
            Classification::NoMatch | Classification::Match(_) | Classification::Error(_) => {
                content_classification
            }
        }
    } else {
        let path_classification = ClassifierTrait::classify(path_classifier, entry, rule);
        match path_classification {
            Classification::NoMatch => path_classification,
            Classification::Match(_) => path_classification,
            Classification::Error(_) => {
                panic!("Classification::Error is not implemented for `PathClassifier`")
            }
            _ => unreachable!("{:?} not possible", path_classification),
        }
    }
}

fn panic_empty() -> ! {
    panic!("Classification::Empty must be checked at the beginning of `get_classification()`")
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::fs::StandaloneFileType;
    use crate::severity::Severity;
    use std::convert::TryInto;

    fn test_classify_entry<D: DirEntryTrait>(entry: &D, rule: &Rule) -> Classification {
        let mut path_classifier = path_classifier::PathClassifier::new(entry);
        let mut content_classifier = content_classifier::ContentClassifier::new(entry);

        classify_entry_with_rule(
            &mut path_classifier,
            &mut content_classifier,
            entry,
            &rule.try_into().unwrap(),
        )
    }

    #[test]
    fn classify_entry_non_existing_file() {
        let entry = StandaloneDirEntry::from_path_with_file_type(
            "not-existing-file.php",
            StandaloneFileType::File,
        );
        let rule = Rule::new_raw(
            "Any PHP",
            Severity::MAJOR,
            "not-existing-file.php",
            false,
            Some("does not matter".to_string()),
        );
        match test_classify_entry(&entry, &rule) {
            Classification::NoMatch => {}
            _ => panic!("Classification must be Classification::NoMatch"),
        }
    }

    #[test]
    fn classify_entry_non_existing_whitelisted_file() {
        let entry = StandaloneDirEntry::from_path_with_file_type(
            "not-existing-file.php",
            StandaloneFileType::File,
        );
        let rule = Rule::new_raw(
            "Any PHP",
            Severity::MAJOR,
            "not-existing-file.php",
            false,
            Some("does not matter".to_string()),
        );
        match test_classify_entry(&entry, &rule) {
            Classification::NoMatch => {}
            _ => panic!("Classification must be Classification::NoMatch"),
        }
    }
}
