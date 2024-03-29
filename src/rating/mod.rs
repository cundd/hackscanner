mod summary;

pub use self::summary::Summary;
use crate::classifier::{classify_entry, Violation};
use crate::dir_entry::DirEntryTrait;
use crate::join::join_violations;
use crate::severity::Severity;
use crate::Rule;
use std::cmp::Reverse;
use std::fmt;

pub fn rate_entries<'a, D: DirEntryTrait>(entries: &'a [D], rules: &'a [Rule]) -> Vec<Rating<'a>> {
    debug!("Will rate entries");
    let result = entries
        .iter()
        .map(|entry| rate_entry(entry, rules))
        .collect();
    debug!("Did rate entries");

    result
}

pub fn rate_entry<'a, D: DirEntryTrait>(entry: &'a D, rules: &[Rule]) -> Rating<'a> {
    info!("Will rate entry {:?}", entry);
    let violations: Vec<Violation> = classify_entry(entry, rules);

    let rating = violations.iter().fold(0, |acc, violation| {
        trace!(
            "  Update rating {} {} {}",
            acc,
            violation.severity() as isize,
            violation.name()
        );

        acc + violation.severity() as isize
    });
    debug!("Did rate entry {:?} (rating: {})", entry, rating);
    Rating::new(entry, rating, violations)
}

pub fn sort_ratings<'a>(ratings: &[Rating<'a>]) -> Vec<Rating<'a>> {
    let mut copy = ratings.to_vec();

    copy.sort_unstable_by_key(|b| Reverse(b.rating()));

    copy
}

#[derive(Debug, Clone)]
pub struct Rating<'a> {
    entry: &'a dyn DirEntryTrait,
    rating: isize,
    violations: Vec<Violation>,
}

impl<'a> Rating<'a> {
    pub fn new(entry: &'a dyn DirEntryTrait, rating: isize, violations: Vec<Violation>) -> Self {
        Rating {
            entry,
            rating,
            violations,
        }
    }

    pub fn entry(&self) -> &dyn DirEntryTrait {
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::fs::StandaloneFileType;
    use crate::severity::Severity;
    use crate::Rule;
    use crate::StandaloneDirEntry;

    fn get_test_dir_entry(file: &str) -> StandaloneDirEntry {
        StandaloneDirEntry::from_path(format!(
            "{}{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/tests/resources/files/",
            file
        ))
        .unwrap()
    }

    mod rate_entry {
        use super::*;
        use crate::RawPath;

        #[test]
        fn rate_entry_test() {
            let entry = get_test_dir_entry("something.tx_mocfilemanager.php");
            let rules = vec![Rule::new(
                "1",
                Severity::NOTICE,
                RawPath::with_path("tx_mocfilemanager"),
                None,
            )
            .unwrap()];

            let rating = rate_entry(&entry, &rules);

            assert_eq!(Severity::NOTICE as isize, rating.rating());
        }

        #[test]
        fn rate_entry_multiple_matches_test() {
            let entry = get_test_dir_entry("something.tx_mocfilemanager.php");
            let rules = vec![
                Rule::new(
                    "2",
                    Severity::MINOR,
                    RawPath::with_path("tx_mocfilemanager"),
                    None,
                )
                .unwrap(),
                Rule::new(
                    "3",
                    Severity::NOTICE,
                    RawPath::with_regex("\\.tx_mocfilemanager"),
                    None,
                )
                .unwrap(),
            ];

            let rating = rate_entry(&entry, &rules);

            assert_eq!(60, rating.rating());
        }

        #[test]
        fn rate_entry_multiple_matches_subtract_test() {
            let entry = get_test_dir_entry("something.tx_mocfilemanager.php");
            let rules = vec![
                Rule::new(
                    "4",
                    Severity::MINOR,
                    RawPath::with_path("tx_mocfilemanager"),
                    None,
                )
                .unwrap(),
                Rule::new(
                    "5",
                    Severity::EASE,
                    RawPath::with_path("tests/resources/files"),
                    None,
                )
                .unwrap(),
            ];

            let rating = rate_entry(&entry, &rules);

            assert_eq!(20, rating.rating());
            assert_eq!(Severity::NOTICE as isize, rating.rating());
        }

        #[test]
        fn rate_entry_with_content_test() {
            let entry = get_test_dir_entry("dezmond.php");
            let rules = vec![Rule::new(
                "6",
                Severity::NOTICE,
                RawPath::with_regex("\\.php"),
                Some("dezmond".to_string()),
            )
            .unwrap()];

            let rating = rate_entry(&entry, &rules);

            assert_eq!(Severity::NOTICE as isize, rating.rating());
        }

        #[test]
        fn rate_entry_non_existing_file() {
            let entry = StandaloneDirEntry::from_path_with_file_type(
                "not-existing-file.php",
                StandaloneFileType::File,
            );
            let rules = vec![Rule::new(
                "Any PHP",
                Severity::MAJOR,
                RawPath::with_path("not-existing-file.php"),
                Some("does not matter".to_string()),
            )
            .unwrap()];

            let rating = rate_entry(&entry, &rules);

            assert_eq!(
                Severity::NONE as isize,
                rating.rating(),
                "Rating {} does not match expected Severity::NOTICE",
                rating.rating()
            );
        }

        #[test]
        fn rate_entry_non_existing_whitelisted_file() {
            let entry = StandaloneDirEntry::from_path_with_file_type(
                "not-existing-file.php",
                StandaloneFileType::File,
            );
            let rules = vec![
                Rule::new(
                    "Any PHP",
                    Severity::MAJOR,
                    RawPath::with_regex("\\.php"),
                    Some("does not matter".to_string()),
                )
                .unwrap(),
                Rule::new(
                    "Whitelisted PHP file",
                    Severity::WHITELIST,
                    RawPath::with_path("not-existing-file.php"),
                    None,
                )
                .unwrap(),
            ];

            let rating = rate_entry(&entry, &rules);

            assert!(
                rating.rating() < 0,
                "Rating {} should be smaller than zero",
                rating.rating()
            );
            assert_eq!("Whitelisted PHP file", rating.violations()[0].name());
        }
    }

    mod rate_entries {
        use super::*;
        use crate::rule::RawPath;

        #[test]
        fn rate_entries_test() {
            let entries = vec![
                get_test_dir_entry("something.tx_mocfilemanager.php"),
                get_test_dir_entry("tx_mocfilemanager.php"),
            ];
            let rules = vec![Rule::new(
                "7",
                Severity::NOTICE,
                RawPath::with_path("tx_mocfilemanager"),
                None,
            )
            .unwrap()];

            let rating = rate_entries(&entries, &rules);

            assert_eq!(
                Severity::NOTICE as isize,
                rating[0].rating(),
                "Rating {} does not match expected Severity::NOTICE",
                rating[0].rating()
            );
            assert_eq!(
                Severity::NOTICE as isize,
                rating[1].rating(),
                "Rating {} does not match expected Severity::NOTICE",
                rating[1].rating()
            );
        }

        #[test]
        fn rate_entries_multiple_matches_test() {
            let entries = vec![
                get_test_dir_entry("something.tx_mocfilemanager.php"),
                get_test_dir_entry("tx_mocfilemanager.php"),
            ];
            let rules = vec![
                Rule::new(
                    "8",
                    Severity::MINOR,
                    RawPath::with_path("tx_mocfilemanager"),
                    None,
                )
                .unwrap(),
                Rule::new(
                    "9",
                    Severity::NOTICE,
                    RawPath::with_regex("\\.tx_mocfilemanager"),
                    None,
                )
                .unwrap(),
            ];

            let rating = rate_entries(&entries, &rules);

            assert_eq!(60, rating[0].rating());
            assert_eq!(
                Severity::MINOR as isize,
                rating[1].rating(),
                "Rating {} does not match expected Severity::MINOR",
                rating[1].rating()
            );
        }

        #[test]
        fn rate_entries_multiple_matches_subtract_test() {
            let entries = vec![
                get_test_dir_entry("something.tx_mocfilemanager.php"),
                get_test_dir_entry("tx_mocfilemanager.php"),
            ];
            let rules = vec![
                Rule::new(
                    "10",
                    Severity::MINOR,
                    RawPath::with_path("tx_mocfilemanager"),
                    None,
                )
                .unwrap(),
                Rule::new(
                    "11",
                    Severity::EASE,
                    RawPath::with_regex("\\.tx_mocfilemanager"),
                    None,
                )
                .unwrap(),
            ];

            let rating = rate_entries(&entries, &rules);

            assert_eq!(
                20,
                rating[0].rating(),
                "Rating {} does not equal expected 20",
                rating[0].rating()
            );
            assert_eq!(
                Severity::NOTICE as isize,
                rating[0].rating(),
                "Rating {} does not match expected Severity::NOTICE",
                rating[0].rating()
            );
            assert_eq!(
                Severity::MINOR as isize,
                rating[1].rating(),
                "Rating {} does not match expected Severity::MINOR",
                rating[1].rating()
            );
        }

        #[test]
        fn rate_entries_with_content_test() {
            let entries = vec![
                get_test_dir_entry("something.tx_mocfilemanager.php"),
                get_test_dir_entry("tx_mocfilemanager.php"),
                get_test_dir_entry("dezmond.php"),
            ];
            let rules = vec![Rule::new(
                "12",
                Severity::MINOR,
                RawPath::with_regex("\\.php"),
                Some("dezmond".to_string()),
            )
            .unwrap()];

            let rating = rate_entries(&entries, &rules);

            assert_eq!(
                0,
                rating[0].rating(),
                "Rating {} does not match expected 0",
                rating[0].rating()
            );
            assert_eq!(
                0,
                rating[1].rating(),
                "Rating {} does not match expected 0",
                rating[1].rating()
            );
            assert_eq!(
                Severity::MINOR as isize,
                rating[2].rating(),
                "Rating {} does not match expected Severity::MINOR",
                rating[2].rating()
            );
        }
    }
}
