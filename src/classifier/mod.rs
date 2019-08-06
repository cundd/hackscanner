mod violation;

use dir_entry::*;
use errors::*;
use matcher::Matcher;
use rating::Rating;
use rule::*;
use std::fs::File;
use std::io::prelude::*;


pub use self::violation::Violation;

/// Number of bytes to read from files
const BUFFER_SIZE: usize = 1024 * 4;

#[allow(unused_imports)]
pub fn classify_entries<'a, 'b, D: DirEntryTrait>(entries: &'a Vec<D>, rules: &'a Vec<PatternRule>) -> Vec<Rating<'a>> {
    debug!("Will classify entries");
    let result = entries.iter()
        .map(|entry| { classify_entry(entry, rules) })
        .collect();
    debug!("Did classify entries");

    result
}

fn classify_entry<'a, 'b, D: DirEntryTrait>(entry: &'a D, rules: &'a Vec<PatternRule>) -> Rating<'a> {
    trace!("Will classify entry {:?}", entry);
    let mut file_content: Option<String> = None;

    let violations: Vec<Violation> = rules.iter().filter_map(|rule| {
        if !Matcher::match_entry_path(rule, entry) {
            return None;
        }

        if !rule.has_content() {
            return Some(Violation::from(rule));
        }

        // Read the entry's content if it is not already loaded
        if file_content.is_none() {
            file_content = match read_entry_content(entry) {
                Ok(s) => Some(s),
                Err(e) => return Some(Violation::from(&e))
            };
        }

        if let Some(ref c) = file_content {
            if !Matcher::match_entry_content(rule, c) {
                return None;
            }
        }

        Some(Violation::from(rule))
    }).collect();


    let rating = violations.iter().fold(0, |acc, violation| {
        trace!("  Update rating {} {} {}", acc, violation.severity() as isize, violation.name());

        acc + violation.severity() as isize
    });
    trace!("Did classify entry {:?} (rating: {})", entry, rating);
    Rating::new(entry, rating, violations)
}


fn read_entry_content<D: DirEntryTrait>(entry: &D) -> Result<String, Error> {
    let path = entry.path();
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => bail!("Could not open file {:?} for reading: {}", entry.path(), e)
    };

    trace!("Will read file {:?}", path);
    let mut buffer = [0; BUFFER_SIZE];
    match file.read(&mut buffer[..]) {
        Ok(bytes_count) => bytes_count,
        Err(e) => {
            bail!("Could not read file {:?}: {}", entry.path(), e)
        }
    };
    trace!("Did read file {:?}", path);

    Ok(String::from_utf8_lossy(&buffer).to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;
    use severity::Severity;

    fn get_test_dir_entry(file: &str) -> StandaloneDirEntry {
        StandaloneDirEntry::from_path(
            PathBuf::from(format!(
                "{}{}{}",
                env!("CARGO_MANIFEST_DIR"),
                "/tests/resources/files/",
                file
            ))
        ).unwrap()
    }

    mod classify_entry {
        use super::*;

        #[test]
        fn classify_entry_test() {
            let entry = get_test_dir_entry("something.tx_mocfilemanager.php");
            let rules = vec![
                Rule::new_raw("1".to_string(), Severity::NOTICE, Some("tx_mocfilemanager".to_owned()), None)
            ];

            let pattern_rules = PatternRule::from_rules_filtered(&rules);
            let rating = classify_entry(&entry, &pattern_rules);

            assert_eq!(Severity::NOTICE as isize, rating.rating());
        }

        #[test]
        fn classify_entry_multiple_matches_test() {
            let entry = get_test_dir_entry("something.tx_mocfilemanager.php");
            let rules = vec![
                Rule::new_raw("2".to_string(), Severity::MINOR, Some("tx_mocfilemanager".to_owned()), None),
                Rule::new_raw("3".to_string(), Severity::NOTICE, Some("\\.tx_mocfilemanager".to_owned()), None)
            ];

            let pattern_rules = PatternRule::from_rules_filtered(&rules);
            let rating = classify_entry(&entry, &pattern_rules);

            assert_eq!(60, rating.rating());
        }

        #[test]
        fn classify_entry_multiple_matches_subtract_test() {
            let entry = get_test_dir_entry("something.tx_mocfilemanager.php");
            let rules = vec![
                Rule::new_raw("4".to_string(), Severity::MINOR, Some("tx_mocfilemanager".to_owned()), None),
                Rule::new_raw("5".to_string(), Severity::EASE, Some("tests/resources/files".to_owned()), None)
            ];

            let pattern_rules = PatternRule::from_rules_filtered(&rules);
            let rating = classify_entry(&entry, &pattern_rules);

            assert_eq!(20, rating.rating());
            assert_eq!(Severity::NOTICE as isize, rating.rating());
        }

        #[test]
        fn classify_entry_with_content_test() {
            let entry = get_test_dir_entry("dezmond.php");
            let rules = vec![
                Rule::new_raw("6".to_string(), Severity::NOTICE, Some("\\.php".to_owned()), Some("dezmond".to_string())),
            ];

            let pattern_rules = PatternRule::from_rules_filtered(&rules);
            let rating = classify_entry(&entry, &pattern_rules);

            assert_eq!(Severity::NOTICE as isize, rating.rating());
        }
    }

    mod classify_entries {
        use super::*;

        #[test]
        fn classify_entries_test() {
            let entries = vec![
                get_test_dir_entry("something.tx_mocfilemanager.php"),
                get_test_dir_entry("tx_mocfilemanager.php"),
            ];
            let rules = vec![
                Rule::new_raw("7".to_string(), Severity::NOTICE, Some("tx_mocfilemanager".to_owned()), None)
            ];

            let pattern_rules = PatternRule::from_rules_filtered(&rules);
            let rating = classify_entries(&entries, &pattern_rules);

            assert_eq!(Severity::NOTICE as isize, rating[0].rating(), "Rating {} does not match expected Severity::NOTICE", rating[0].rating());
            assert_eq!(Severity::NOTICE as isize, rating[1].rating(), "Rating {} does not match expected Severity::NOTICE", rating[1].rating());
        }

        #[test]
        fn classify_entries_multiple_matches_test() {
            let entries = vec![
                get_test_dir_entry("something.tx_mocfilemanager.php"),
                get_test_dir_entry("tx_mocfilemanager.php"),
            ];
            let rules = vec![
                Rule::new_raw("8".to_string(), Severity::MINOR, Some("tx_mocfilemanager".to_owned()), None),
                Rule::new_raw("9".to_string(), Severity::NOTICE, Some("\\.tx_mocfilemanager".to_owned()), None)
            ];

            let pattern_rules = PatternRule::from_rules_filtered(&rules);
            let rating = classify_entries(&entries, &pattern_rules);

            assert_eq!(60, rating[0].rating());
            assert_eq!(Severity::MINOR as isize, rating[1].rating(), "Rating {} does not match expected Severity::MINOR", rating[1].rating());
        }

        #[test]
        fn classify_entries_multiple_matches_subtract_test() {
            let entries = vec![
                get_test_dir_entry("something.tx_mocfilemanager.php"),
                get_test_dir_entry("tx_mocfilemanager.php"),
            ];
            let rules = vec![
                Rule::new_raw("10".to_string(), Severity::MINOR, Some("tx_mocfilemanager".to_owned()), None),
                Rule::new_raw("11".to_string(), Severity::EASE, Some("\\.tx_mocfilemanager".to_owned()), None)
            ];

            let pattern_rules = PatternRule::from_rules_filtered(&rules);
            let rating = classify_entries(&entries, &pattern_rules);

            assert_eq!(20, rating[0].rating(), "Rating {} does not equal expected 20", rating[0].rating());
            assert_eq!(Severity::NOTICE as isize, rating[0].rating(), "Rating {} does not match expected Severity::NOTICE", rating[0].rating());
            assert_eq!(Severity::MINOR as isize, rating[1].rating(), "Rating {} does not match expected Severity::MINOR", rating[1].rating());
        }

        #[test]
        fn classify_entries_with_content_test() {
            let entries = vec![
                get_test_dir_entry("something.tx_mocfilemanager.php"),
                get_test_dir_entry("tx_mocfilemanager.php"),
                get_test_dir_entry("dezmond.php"),
            ];
            let rules = vec![
                Rule::new_raw("12".to_string(), Severity::MINOR, Some("\\.php".to_owned()), Some("dezmond".to_string())),
            ];

            let pattern_rules = PatternRule::from_rules_filtered(&rules);
            let rating = classify_entries(&entries, &pattern_rules);

            assert_eq!(0, rating[0].rating(), "Rating {} does not match expected 0", rating[0].rating());
            assert_eq!(0, rating[1].rating(), "Rating {} does not match expected 0", rating[1].rating());
            assert_eq!(Severity::MINOR as isize, rating[2].rating(), "Rating {} does not match expected Severity::MINOR", rating[2].rating());
        }
    }
}
