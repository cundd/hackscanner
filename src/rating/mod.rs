mod summary;

pub use self::summary::Summary;
use std::fmt;
use dir_entry::DirEntryTrait;
use severity::Severity;
use classifier::{Violation, classify_entry};
use join::join_violations;
use ::PatternRule;

pub fn rate_entries<'a, 'b, D: DirEntryTrait>(entries: &'a Vec<D>, rules: &'a Vec<PatternRule>) -> Vec<Rating<'a>> {
    debug!("Will rate entries");
    let result = entries.iter()
        .map(|entry| { rate_entry(entry, rules) })
        .collect();
    debug!("Did rate entries");

    result
}

pub fn sort_ratings<'a>(ratings: &[Rating<'a>]) -> Vec<Rating<'a>> {
    let mut copy = ratings.to_vec();

    copy.sort_unstable_by(|a, b| b.rating().cmp(&a.rating()));

    copy
}

#[derive(Debug, Clone)]
pub struct Rating<'a> {
    entry: &'a DirEntryTrait,
    rating: isize,
    violations: Vec<Violation>,
}

impl<'a> Rating<'a> {
    pub fn new(entry: &'a DirEntryTrait, rating: isize, violations: Vec<Violation>) -> Self {
        Rating {
            entry,
            rating,
            violations,
        }
    }

    pub fn entry(&self) -> &DirEntryTrait {
        self.entry
    }

    pub fn violations(&self) -> &Vec<Violation> {
        &self.violations
    }

    pub fn rating(&self) -> isize {
        self.rating
    }

    pub fn rating_description(&self) -> &'static str {
        if self.rating >= Severity::CRITICAL as isize {
            "CRITICAL"
        } else if self.rating >= Severity::MAJOR as isize {
            "MAJOR"
        } else if self.rating >= Severity::MINOR as isize {
            "MINOR"
        } else if self.rating >= Severity::NOTICE as isize {
            "NOTICE"
        } else {
            "CLEAN"
        }
    }

    pub fn description(&self) -> String {
        let path_as_string: String = self.entry.path().to_string_lossy().into_owned();

        format!(
            "[{}] {} (Rules: {})",
            self.rating_description(),
            path_as_string,
            join_violations(&self.violations)
        )
    }
}

impl<'a> fmt::Display for Rating<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

fn rate_entry<'a, 'b, D: DirEntryTrait>(entry: &'a D, rules: &'a Vec<PatternRule>) -> Rating<'a> {
    trace!("Will rate entry {:?}", entry);
    let violations: Vec<Violation> = classify_entry(entry, rules);

    let rating = violations.iter().fold(0, |acc, violation| {
        trace!("  Update rating {} {} {}", acc, violation.severity() as isize, violation.name());

        acc + violation.severity() as isize
    });
    trace!("Did rate entry {:?} (rating: {})", entry, rating);
    Rating::new(entry, rating, violations)
}

#[cfg(test)]
mod test {
    use super::*;
    use severity::Severity;
    use StandaloneDirEntry;
    use fs::StandaloneFileType;
    use Rule;

    fn get_test_dir_entry(file: &str) -> StandaloneDirEntry {
        StandaloneDirEntry::from_path(
            format!(
                "{}{}{}",
                env!("CARGO_MANIFEST_DIR"),
                "/tests/resources/files/",
                file
            )
        ).unwrap()
    }

    mod rate_entry {
        use super::*;

        #[test]
        fn rate_entry_test() {
            let entry = get_test_dir_entry("something.tx_mocfilemanager.php");
            let rules = vec![
                Rule::new_raw("1".to_string(), Severity::NOTICE, Some("tx_mocfilemanager".to_owned()), None)
            ];

            let pattern_rules = PatternRule::from_rules_filtered(&rules);
            let rating = rate_entry(&entry, &pattern_rules);

            assert_eq!(Severity::NOTICE as isize, rating.rating());
        }

        #[test]
        fn rate_entry_multiple_matches_test() {
            let entry = get_test_dir_entry("something.tx_mocfilemanager.php");
            let rules = vec![
                Rule::new_raw("2".to_string(), Severity::MINOR, Some("tx_mocfilemanager".to_owned()), None),
                Rule::new_raw("3".to_string(), Severity::NOTICE, Some("\\.tx_mocfilemanager".to_owned()), None)
            ];

            let pattern_rules = PatternRule::from_rules_filtered(&rules);
            let rating = rate_entry(&entry, &pattern_rules);

            assert_eq!(60, rating.rating());
        }

        #[test]
        fn rate_entry_multiple_matches_subtract_test() {
            let entry = get_test_dir_entry("something.tx_mocfilemanager.php");
            let rules = vec![
                Rule::new_raw("4".to_string(), Severity::MINOR, Some("tx_mocfilemanager".to_owned()), None),
                Rule::new_raw("5".to_string(), Severity::EASE, Some("tests/resources/files".to_owned()), None)
            ];

            let pattern_rules = PatternRule::from_rules_filtered(&rules);
            let rating = rate_entry(&entry, &pattern_rules);

            assert_eq!(20, rating.rating());
            assert_eq!(Severity::NOTICE as isize, rating.rating());
        }

        #[test]
        fn rate_entry_with_content_test() {
            let entry = get_test_dir_entry("dezmond.php");
            let rules = vec![
                Rule::new_raw("6".to_string(), Severity::NOTICE, Some("\\.php".to_owned()), Some("dezmond".to_string())),
            ];

            let pattern_rules = PatternRule::from_rules_filtered(&rules);
            let rating = rate_entry(&entry, &pattern_rules);

            assert_eq!(Severity::NOTICE as isize, rating.rating());
        }

        #[test]
        fn rate_entry_non_existing_file() {
            let entry = StandaloneDirEntry::from_path_with_file_type("not-existing-file.php", StandaloneFileType::File);
            let rules = vec![
                Rule::new_raw("Any PHP".to_string(), Severity::MAJOR, None, Some("does not matter".to_string())),
            ];

            let pattern_rules = PatternRule::from_rules_filtered(&rules);
            let rating = rate_entry(&entry, &pattern_rules);

            assert_eq!(
                Severity::NOTICE as isize,
                rating.rating(),
                "Rating {} does not match expected Severity::NOTICE", rating.rating()
            );
            assert_eq!(
                "Could not open file \"not-existing-file.php\" for reading: No such file or directory (os error 2)",
                rating.violations()[0].name()
            );
        }
    }

    mod rate_entries {
        use super::*;

        #[test]
        fn rate_entries_test() {
            let entries = vec![
                get_test_dir_entry("something.tx_mocfilemanager.php"),
                get_test_dir_entry("tx_mocfilemanager.php"),
            ];
            let rules = vec![
                Rule::new_raw("7".to_string(), Severity::NOTICE, Some("tx_mocfilemanager".to_owned()), None)
            ];

            let pattern_rules = PatternRule::from_rules_filtered(&rules);
            let rating = rate_entries(&entries, &pattern_rules);

            assert_eq!(Severity::NOTICE as isize, rating[0].rating(), "Rating {} does not match expected Severity::NOTICE", rating[0].rating());
            assert_eq!(Severity::NOTICE as isize, rating[1].rating(), "Rating {} does not match expected Severity::NOTICE", rating[1].rating());
        }

        #[test]
        fn rate_entries_multiple_matches_test() {
            let entries = vec![
                get_test_dir_entry("something.tx_mocfilemanager.php"),
                get_test_dir_entry("tx_mocfilemanager.php"),
            ];
            let rules = vec![
                Rule::new_raw("8".to_string(), Severity::MINOR, Some("tx_mocfilemanager".to_owned()), None),
                Rule::new_raw("9".to_string(), Severity::NOTICE, Some("\\.tx_mocfilemanager".to_owned()), None)
            ];

            let pattern_rules = PatternRule::from_rules_filtered(&rules);
            let rating = rate_entries(&entries, &pattern_rules);

            assert_eq!(60, rating[0].rating());
            assert_eq!(Severity::MINOR as isize, rating[1].rating(), "Rating {} does not match expected Severity::MINOR", rating[1].rating());
        }

        #[test]
        fn rate_entries_multiple_matches_subtract_test() {
            let entries = vec![
                get_test_dir_entry("something.tx_mocfilemanager.php"),
                get_test_dir_entry("tx_mocfilemanager.php"),
            ];
            let rules = vec![
                Rule::new_raw("10".to_string(), Severity::MINOR, Some("tx_mocfilemanager".to_owned()), None),
                Rule::new_raw("11".to_string(), Severity::EASE, Some("\\.tx_mocfilemanager".to_owned()), None)
            ];

            let pattern_rules = PatternRule::from_rules_filtered(&rules);
            let rating = rate_entries(&entries, &pattern_rules);

            assert_eq!(20, rating[0].rating(), "Rating {} does not equal expected 20", rating[0].rating());
            assert_eq!(Severity::NOTICE as isize, rating[0].rating(), "Rating {} does not match expected Severity::NOTICE", rating[0].rating());
            assert_eq!(Severity::MINOR as isize, rating[1].rating(), "Rating {} does not match expected Severity::MINOR", rating[1].rating());
        }

        #[test]
        fn rate_entries_with_content_test() {
            let entries = vec![
                get_test_dir_entry("something.tx_mocfilemanager.php"),
                get_test_dir_entry("tx_mocfilemanager.php"),
                get_test_dir_entry("dezmond.php"),
            ];
            let rules = vec![
                Rule::new_raw("12".to_string(), Severity::MINOR, Some("\\.php".to_owned()), Some("dezmond".to_string())),
            ];

            let pattern_rules = PatternRule::from_rules_filtered(&rules);
            let rating = rate_entries(&entries, &pattern_rules);

            assert_eq!(0, rating[0].rating(), "Rating {} does not match expected 0", rating[0].rating());
            assert_eq!(0, rating[1].rating(), "Rating {} does not match expected 0", rating[1].rating());
            assert_eq!(Severity::MINOR as isize, rating[2].rating(), "Rating {} does not match expected Severity::MINOR", rating[2].rating());
        }
    }
}
