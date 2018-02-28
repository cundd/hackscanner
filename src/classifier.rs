use std::fs::File;
use std::io::prelude::*;

use rule::*;
use dir_entry::*;
use errors::*;
use rating::Rating;
use matcher::Matcher;

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
    let mut rating: isize = 0;
    let content = match read_entry_content(entry) {
        Ok(s) => s,
        Err(e) => {
            error!("{}", e);
            "".to_string()
        }
    };

    let matching_rules = rules.iter().filter(|rule| {
        if !Matcher::match_entry_path(rule, entry) {
            return false;
        }

        if rule.content().is_some() && !Matcher::match_entry_content(rule, &content) {
            return false;
        }

        rating += rule.severity() as isize;

        return true;
    }).collect();
    trace!("Did classify entry {:?}", entry);

    Rating::new(entry, rating, matching_rules)
}


fn read_entry_content<'a, D: DirEntryTrait>(entry: &'a D) -> Result<String, Error> {
    let mut file = match File::open(entry.path()) {
        Ok(f) => f,
        Err(e) => bail!("Could not open file for reading: {}", e)
    };

//    trace!("Will read file {:?}", entry.path());
//    let mut contents = String::new();
//    file.read_to_string(&mut contents)
//        .expect("something went wrong reading the file");
//    trace!("Did read file {:?}", entry.path());
//
//    return contents;

    trace!("Will read file {:?}", entry.path());
    let mut buffer = [0; BUFFER_SIZE];
    match file.read(&mut buffer[..]) {
        Ok(bytes_count) => bytes_count,
        Err(e) => {
            bail!("Could not read file: {}", e)
        }
    };
    trace!("Did read file {:?}", entry.path());

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
                Rule::new(1, Severity::NOTICE, Some("tx_mocfilemanager".to_owned()), None)
            ];

            let pattern_rules = PatternRule::from_rules_filtered(&rules);
            let rating = classify_entry(&entry, &pattern_rules);

            assert_eq!(Severity::NOTICE as isize, rating.rating());
        }

        #[test]
        fn classify_entry_multiple_matches_test() {
            let entry = get_test_dir_entry("something.tx_mocfilemanager.php");
            let rules = vec![
                Rule::new(2, Severity::MINOR, Some("tx_mocfilemanager".to_owned()), None),
                Rule::new(3, Severity::NOTICE, Some("\\.tx_mocfilemanager".to_owned()), None)
            ];

            let pattern_rules = PatternRule::from_rules_filtered(&rules);
            let rating = classify_entry(&entry, &pattern_rules);

            assert_eq!(60, rating.rating());
        }

        #[test]
        fn classify_entry_multiple_matches_subtract_test() {
            let entry = get_test_dir_entry("something.tx_mocfilemanager.php");
            let rules = vec![
                Rule::new(4, Severity::MINOR, Some("tx_mocfilemanager".to_owned()), None),
                Rule::new(5, Severity::EASE, Some("tests/resources/files".to_owned()), None)
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
                Rule::new(6, Severity::NOTICE, Some("\\.php".to_owned()), Some("dezmond".to_string())),
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
                Rule::new(7, Severity::NOTICE, Some("tx_mocfilemanager".to_owned()), None)
            ];

            let pattern_rules = PatternRule::from_rules_filtered(&rules);
            let rating = classify_entries(&entries, &pattern_rules);

            assert_eq!(Severity::NOTICE as isize, rating[0].rating());
            assert_eq!(Severity::NOTICE as isize, rating[1].rating());
        }

        #[test]
        fn classify_entries_multiple_matches_test() {
            let entries = vec![
                get_test_dir_entry("something.tx_mocfilemanager.php"),
                get_test_dir_entry("tx_mocfilemanager.php"),
            ];
            let rules = vec![
                Rule::new(8, Severity::MINOR, Some("tx_mocfilemanager".to_owned()), None),
                Rule::new(9, Severity::NOTICE, Some("\\.tx_mocfilemanager".to_owned()), None)
            ];

            let pattern_rules = PatternRule::from_rules_filtered(&rules);
            let rating = classify_entries(&entries, &pattern_rules);

            assert_eq!(60, rating[0].rating());
            assert_eq!(Severity::MINOR as isize, rating[1].rating());
        }

        #[test]
        fn classify_entries_multiple_matches_subtract_test() {
            let entries = vec![
                get_test_dir_entry("something.tx_mocfilemanager.php"),
                get_test_dir_entry("tx_mocfilemanager.php"),
            ];
            let rules = vec![
                Rule::new(10, Severity::MINOR, Some("tx_mocfilemanager".to_owned()), None),
                Rule::new(11, Severity::EASE, Some("\\.tx_mocfilemanager".to_owned()), None)
            ];

            let pattern_rules = PatternRule::from_rules_filtered(&rules);
            let rating = classify_entries(&entries, &pattern_rules);

            assert_eq!(20, rating[0].rating());
            assert_eq!(Severity::NOTICE as isize, rating[0].rating());
            assert_eq!(Severity::MINOR as isize, rating[1].rating());
        }

        #[test]
        fn classify_entries_with_content_test() {
            let entries = vec![
                get_test_dir_entry("something.tx_mocfilemanager.php"),
                get_test_dir_entry("tx_mocfilemanager.php"),
                get_test_dir_entry("dezmond.php"),
            ];
            let rules = vec![
                Rule::new(12, Severity::MINOR, Some("\\.php".to_owned()), Some("dezmond".to_string())),
            ];

            let pattern_rules = PatternRule::from_rules_filtered(&rules);
            let rating = classify_entries(&entries, &pattern_rules);

            assert_eq!(0, rating[0].rating());
            assert_eq!(0, rating[1].rating());
            assert_eq!(Severity::MINOR as isize, rating[2].rating());
        }
    }
}