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

pub fn classify_entries<D: DirEntryTrait>(entries: &[D], rules: &[Rule]) -> Vec<Vec<Violation>> {
    debug!("Will classify entries");
    let result = entries
        .iter()
        .map(|entry| classify_entry(entry, rules))
        .collect();
    debug!("Did classify entries");

    result
}

pub fn classify_entry<D: DirEntryTrait>(entry: &D, rules: &[Rule]) -> Vec<Violation> {
    let mut path_classifier = path_classifier::PathClassifier::new(entry);
    let mut content_classifier = content_classifier::ContentClassifier::new(entry);
    rules
        .iter()
        .filter_map(|rule| {
            match classify_entry_with_rule(
                &mut path_classifier,
                &mut content_classifier,
                entry,
                rule,
            ) {
                Classification::NoMatch => None,
                Classification::Match(violation) => Some(violation),
                Classification::Error(violation) => Some(violation),
            }
        })
        .collect()
}

trait ClassifierTrait<D: DirEntryTrait> {
    fn new(entry: &D) -> Self;
    fn classify(&mut self, entry: &D, rule: &Rule) -> Classification;
}

fn classify_entry_with_rule<D: DirEntryTrait>(
    path_classifier: &mut PathClassifier,
    content_classifier: &mut ContentClassifier,
    entry: &D,
    rule: &Rule,
) -> Classification {
    if rule.has_content() {
        let path_classification = ClassifierTrait::classify(path_classifier, entry, rule);
        match path_classification {
            Classification::NoMatch => {
                /* Path does not match. No need to check the content */
                Classification::NoMatch
            }
            Classification::Match(_) => {
                /* Path does match. Now check the content */
                ClassifierTrait::classify(content_classifier, entry, rule)
            }
            Classification::Error(_) => {
                unreachable!("Classification::Error is not implemented for `PathClassifier`")
            }
        }
    } else {
        let path_classification = ClassifierTrait::classify(path_classifier, entry, rule);

        match path_classification {
            Classification::Error(_) => {
                unreachable!("Classification::Error is not implemented for `PathClassifier`")
            }
            _ => path_classification,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::fs::StandaloneFileType;
    use crate::severity::Severity;

    fn test_classify_entry<D: DirEntryTrait>(entry: &D, rule: &Rule) -> Classification {
        let mut path_classifier = path_classifier::PathClassifier::new(entry);
        let mut content_classifier = content_classifier::ContentClassifier::new(entry);

        classify_entry_with_rule(&mut path_classifier, &mut content_classifier, entry, rule)
    }

    #[test]
    fn classify_entry_non_existing_file() {
        let entry = StandaloneDirEntry::from_path_with_file_type(
            "not-existing-file.php",
            StandaloneFileType::File,
        );
        let rule = Rule::new(
            "Any PHP",
            Severity::MAJOR,
            RawPath::with_path("not-existing-file.php"),
            Some("does not matter".to_string()),
        )
        .unwrap();
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
        let rule = Rule::new(
            "Any PHP",
            Severity::MAJOR,
            RawPath::with_path("not-existing-file.php"),
            Some("does not matter".to_string()),
        )
        .unwrap();
        match test_classify_entry(&entry, &rule) {
            Classification::NoMatch => {}
            _ => panic!("Classification must be Classification::NoMatch"),
        }
    }
}
