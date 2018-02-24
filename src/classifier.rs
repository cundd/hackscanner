#[allow(unused_imports)]
use std::path::Path;
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use walkdir::WalkDir;
#[allow(unused_imports)]
use rule::*;
use dir_entry::*;
use rating::Rating;
use matcher::Matcher;


#[allow(unused_imports)]
pub fn classify_entries<'a, D: DirEntryTrait>(entries: &'a Vec<D>, rules: &Vec<PatternRule>) -> Vec<Rating<'a>> {
    entries.iter()
        .map(|entry| { classify_entry(entry, rules) })
        .collect()
}

#[allow(unused)]
fn classify_entry<'a, D: DirEntryTrait>(entry: &'a D, rules: &Vec<PatternRule>) -> Rating<'a> {
    let mut rating: i32 = 0;
    for rule in rules {
        if !Matcher::match_entry_path(rule, entry) {
            continue;
        }

        if let Some(content) = rule.content() {
            // TODO: Check the content
        } else {
            rating += rule.score() as i32
        }
    }
    Rating::new(entry, rating)
}


#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

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
                Rule::new(Some("tx_mocfilemanager".to_owned()), None, 8)
            ];

            let pattern_rules = PatternRule::from_rules_filtered(&rules);
            let rating = classify_entry(&entry, &pattern_rules);

            assert_eq!(8, rating.rating());
        }

        #[test]
        fn classify_entry_multiple_matches_test() {
            let entry = get_test_dir_entry("something.tx_mocfilemanager.php");
            let rules = vec![
                Rule::new(Some("tx_mocfilemanager".to_owned()), None, 8),
                Rule::new(Some("\\.tx_mocfilemanager".to_owned()), None, 4)
            ];

            let pattern_rules = PatternRule::from_rules_filtered(&rules);
            let rating = classify_entry(&entry, &pattern_rules);

            assert_eq!(12, rating.rating());
        }

        #[test]
        fn classify_entry_multiple_matches_subtract_test() {
            let entry = get_test_dir_entry("something.tx_mocfilemanager.php");
            let rules = vec![
                Rule::new(Some("tx_mocfilemanager".to_owned()), None, 8),
                Rule::new(Some("tests/resources/files".to_owned()), None, -4)
            ];

            let pattern_rules = PatternRule::from_rules_filtered(&rules);
            let rating = classify_entry(&entry, &pattern_rules);

            assert_eq!(4, rating.rating());
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
                Rule::new(Some("tx_mocfilemanager".to_owned()), None, 8)
            ];

            let pattern_rules = PatternRule::from_rules_filtered(&rules);
            let rating = classify_entries(&entries, &pattern_rules);

            assert_eq!(8, rating[0].rating());
            assert_eq!(8, rating[1].rating());
        }

        #[test]
        fn classify_entries_multiple_matches_test() {
            let entries = vec![
                get_test_dir_entry("something.tx_mocfilemanager.php"),
                get_test_dir_entry("tx_mocfilemanager.php"),
            ];
            let rules = vec![
                Rule::new(Some("tx_mocfilemanager".to_owned()), None, 8),
                Rule::new(Some("\\.tx_mocfilemanager".to_owned()), None, 4)
            ];

            let pattern_rules = PatternRule::from_rules_filtered(&rules);
            let rating = classify_entries(&entries, &pattern_rules);

            assert_eq!(12, rating[0].rating());
            assert_eq!(8, rating[1].rating());
        }

        #[test]
        fn classify_entries_multiple_matches_subtract_test() {
            let entries = vec![
                get_test_dir_entry("something.tx_mocfilemanager.php"),
                get_test_dir_entry("tx_mocfilemanager.php"),
            ];
            let rules = vec![
                Rule::new(Some("tx_mocfilemanager".to_owned()), None, 8),
                Rule::new(Some("\\.tx_mocfilemanager".to_owned()), None, -4)
            ];

            let pattern_rules = PatternRule::from_rules_filtered(&rules);
            let rating = classify_entries(&entries, &pattern_rules);

            assert_eq!(4, rating[0].rating());
            assert_eq!(8, rating[1].rating());
        }
    }
}