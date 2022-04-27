use crate::errors::*;
use crate::rule::pattern_rule::PatternRule;
use crate::rule::raw_rule::RawRule;
use std::error::Error as StdError;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct Reader {}

impl Reader {
    pub fn read_rules_from_file(path: &Path) -> Result<Vec<PatternRule>> {
        let raw_rules = Reader::read_raw_rules_from_file(path)?;

        PatternRule::from_raw_rules(raw_rules)
    }

    fn read_raw_rules_from_file(path: &Path) -> Result<Vec<RawRule>> {
        match path.extension() {
            None => Err(build_file_type_error(path)),
            Some(os_str) => match os_str.to_str() {
                None => Err(build_file_type_error(path)),

                #[cfg(feature = "json")]
                Some("json") => Reader::read_rules_from_json_file(path),

                #[cfg(feature = "yaml")]
                Some("yaml") => Reader::read_rules_from_yaml_file(path),

                Some(t) => bail!("No deserializer for the file type '{}'", t),
            },
        }
    }

    #[cfg(feature = "json")]
    fn read_rules_from_json_file(path: &Path) -> Result<Vec<RawRule>> {
        let file: BufReader<File> = get_file_reader(path)?;
        match serde_json::from_reader::<BufReader<File>, Vec<RawRule>>(file) {
            Ok(r) => Ok(r),
            Err(e) => Err(build_deserialize_error(path, &e)),
        }
    }

    #[cfg(feature = "yaml")]
    fn read_rules_from_yaml_file(path: &Path) -> Result<Vec<RawRule>> {
        let file: BufReader<File> = get_file_reader(path)?;
        match serde_yaml::from_reader::<BufReader<File>, Vec<RawRule>>(file) {
            Ok(r) => Ok(r),
            Err(e) => Err(build_deserialize_error(path, &e)),
        }
    }
}

fn build_file_type_error(path: &Path) -> Error {
    ErrorKind::ReaderError(format!(
        "Could not detect the file type of '{}'",
        path.display()
    ))
    .into()
}

fn build_deserialize_error(path: &Path, error: &dyn StdError) -> Error {
    ErrorKind::ReaderError(format!(
        "Could not deserialize the file '{}': {}",
        path.display(),
        error
    ))
    .into()
}

fn get_file_reader(path: &Path) -> Result<BufReader<File>, Error> {
    match File::open(path) {
        Ok(f) => Ok(BufReader::new(f)),
        Err(e) => Err(ErrorKind::ReaderError(format!(
            "Could not open file '{}' for reading: {}",
            path.display(),
            e
        ))
        .into()),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::severity::Severity;

    fn path(rule: &RawRule) -> String {
        format!("{}", rule.path())
    }

    fn content(rule: &RawRule) -> String {
        rule.content().unwrap()
    }

    fn test_loaded_rules(result: Result<Vec<RawRule>, Error>) {
        assert!(result.is_ok(), "{}", result.unwrap_err());
        let rules = result.unwrap();

        assert_eq!(2, rules.len());

        assert_eq!("some rule", rules[0].name());
        assert_eq!("some/path", path(&rules[0]));
        assert_eq!("some bad content", content(&rules[0]));
        assert_eq!(Severity::CRITICAL, rules[0].severity());

        assert_eq!("some whitelist rule", rules[1].name());
        assert_eq!("\\.php", path(&rules[1]));
        assert_eq!("love", content(&rules[1]));
        assert_eq!(Severity::WHITELIST, rules[1].severity());
    }

    #[test]
    fn read_raw_rules_from_file_invalid() {
        let result = Reader::read_raw_rules_from_file(Path::new(&format!(
            "{}/tests",
            env!("CARGO_MANIFEST_DIR")
        )));
        assert!(result.is_err());
        assert_eq!(
            "Invalid configuration file",
            result.unwrap_err().description()
        );

        let result = Reader::read_raw_rules_from_file(Path::new(&format!(
            "{}/tests.txt",
            env!("CARGO_MANIFEST_DIR")
        )));
        assert!(result.is_err());
        assert_eq!(
            "No deserializer for the file type 'txt'",
            result.unwrap_err().description()
        );
    }

    #[test]
    #[cfg(feature = "json")]
    fn read_raw_rules_from_file_with_not_existing_json() {
        let result = Reader::read_raw_rules_from_file(Path::new(&format!(
            "{}/tests/resources/rules/not-a-file.json",
            env!("CARGO_MANIFEST_DIR")
        )));
        assert!(result.is_err());
        assert_eq!(
            "Invalid configuration file",
            result.unwrap_err().description()
        );
    }

    #[test]
    #[cfg(feature = "json")]
    fn read_raw_rules_from_file_with_json() {
        let result = Reader::read_raw_rules_from_file(Path::new(&format!(
            "{}/tests/resources/rules/rules.json",
            env!("CARGO_MANIFEST_DIR")
        )));
        test_loaded_rules(result);
    }

    #[test]
    #[cfg(feature = "yaml")]
    fn read_raw_rules_from_file_with_not_existing_yaml() {
        let result = Reader::read_raw_rules_from_file(Path::new(&format!(
            "{}/tests/resources/rules/not-a-file.yaml",
            env!("CARGO_MANIFEST_DIR")
        )));
        assert!(result.is_err());
        assert_eq!(
            "Invalid configuration file",
            result.unwrap_err().description()
        );
    }

    #[test]
    #[cfg(feature = "yaml")]
    fn read_raw_rules_from_file_with_yaml() {
        let result = Reader::read_raw_rules_from_file(Path::new(&format!(
            "{}/tests/resources/rules/rules.yaml",
            env!("CARGO_MANIFEST_DIR")
        )));
        test_loaded_rules(result);
    }
}
