use crate::rule::raw_rule::RawRule;
use std::path::Path;
use crate::errors::*;
use std::fs::File;
use std::io::BufReader;
use crate::Rule;
use std::error::Error as StdError;

pub struct Reader {}

impl Reader {
    pub fn read_rules_from_file(path: &Path) -> Result<Vec<Rule>, Error> {
        match path.extension() {
            None => Err(build_file_type_error(path)),
            Some(os_str) => {
                match os_str.to_str() {
                    None => Err(build_file_type_error(path)),

                    #[cfg(feature = "json")]
                    Some("json") => Reader::read_rules_from_json_file(path),

                    #[cfg(feature = "yaml")]
                    Some("yaml") => Reader::read_rules_from_yaml_file(path),

                    Some(t) => bail!("No deserializer for the file type '{}'", t),
                }
            }
        }
    }

    #[cfg(feature = "json")]
    fn read_rules_from_json_file(path: &Path) -> Result<Vec<Rule>, Error> {
        extern crate serde_json;

        let file: BufReader<File> = get_file_reader(path)?;
        let raw = match serde_json::from_reader::<BufReader<File>, Vec<RawRule>>(file) {
            Ok(r) => (r),
            Err(e) => return Err(build_deserialize_error(path, &e))
        };

        Ok(raw.into_iter().map(|r| Rule::RawRule(r)).collect())
    }

    #[cfg(feature = "yaml")]
    fn read_rules_from_yaml_file(path: &Path) -> Result<Vec<Rule>, Error> {
        extern crate serde_yaml;

        let file: BufReader<File> = get_file_reader(path)?;
        let raw = match serde_yaml::from_reader::<BufReader<File>, Vec<RawRule>>(file) {
            Ok(r) => (r),
            Err(e) => return Err(build_deserialize_error(path, &e))
        };

        Ok(raw.into_iter().map(|r| Rule::RawRule(r)).collect())
    }
}

fn build_file_type_error(path: &Path) -> Error {
    match path.to_str() {
        None => ErrorKind::ReaderError("Invalid file".to_string()),
        Some(f) => ErrorKind::ReaderError(format!("Could not detect the file type of '{}'", f)),
    }.into()
}

fn build_deserialize_error(path: &Path, error: &dyn StdError) -> Error {
    match path.to_str() {
        None => ErrorKind::ReaderError(format!("Could not deserialize file: {}", error)),
        Some(f) => ErrorKind::ReaderError(format!("Could not deserialize the file '{}': {}", f, error)),
    }.into()
}

fn get_file_reader(path: &Path) -> Result<BufReader<File>, Error> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            return Err(
                ErrorKind::ReaderError(
                    format!("Could not open file {:?} for reading: {}", path, e)
                ).into()
            );
        }
    };
    Ok(BufReader::new(file))
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::rule::RuleTrait;
    use crate::severity::Severity;

    fn path(rule: &Rule) -> String {
        match rule {
            &Rule::RawRule(ref rule) => rule.path().unwrap().to_string(),
            &Rule::PatternRule(ref rule) => rule.path().unwrap().to_string(),
        }
    }

    fn content(rule: &Rule) -> String {
        match rule {
            &Rule::RawRule(ref rule) => rule.content().unwrap().to_string(),
            &Rule::PatternRule(ref rule) => rule.content().unwrap().to_string(),
        }
    }

    fn test_loaded_rules(result: Result<Vec<Rule>, Error>) {
        assert!(result.is_ok(), "{}", result.unwrap_err());
        let rules = result.unwrap();

        assert_eq!(2, rules.len());

        assert_eq!("some rule", rules[0].name().clone());
        assert_eq!("some/path", path(&rules[0]));
        assert_eq!("some bad content", content(&rules[0]));
        assert_eq!(Severity::CRITICAL, rules[0].severity());

        assert_eq!("some whitelist rule", rules[1].name().clone());
        assert_eq!("\\.php", path(&rules[1]));
        assert_eq!("love", content(&rules[1]));
        assert_eq!(Severity::WHITELIST, rules[1].severity());
    }

    #[test]
    fn read_rules_from_file_invalid() {
        let result = Reader::read_rules_from_file(
            &Path::new(&format!("{}/tests", env!("CARGO_MANIFEST_DIR")))
        );
        assert!(result.is_err());
        assert_eq!("Invalid configuration file", result.unwrap_err().description());

        let result = Reader::read_rules_from_file(
            &Path::new(&format!("{}/tests.txt", env!("CARGO_MANIFEST_DIR")))
        );
        assert!(result.is_err());
        assert_eq!("No deserializer for the file type 'txt'", result.unwrap_err().description());
    }

    #[test]
    #[cfg(feature = "json")]
    fn read_rules_from_file_with_not_existing_json() {
        let result = Reader::read_rules_from_file(
            &Path::new(&format!("{}/tests/resources/rules/not-a-file.json", env!("CARGO_MANIFEST_DIR")))
        );
        assert!(result.is_err());
        assert_eq!("Invalid configuration file", result.unwrap_err().description());
    }

    #[test]
    #[cfg(feature = "json")]
    fn read_rules_from_file_with_json() {
        let result = Reader::read_rules_from_file(
            &Path::new(&format!("{}/tests/resources/rules/rules.json", env!("CARGO_MANIFEST_DIR")))
        );
        test_loaded_rules(result);
    }

    #[test]
    #[cfg(feature = "yaml")]
    fn read_rules_from_file_with_not_existing_yaml() {
        let result = Reader::read_rules_from_file(
            &Path::new(&format!("{}/tests/resources/rules/not-a-file.yaml", env!("CARGO_MANIFEST_DIR")))
        );
        assert!(result.is_err());
        assert_eq!("Invalid configuration file", result.unwrap_err().description());
    }

    #[test]
    #[cfg(feature = "yaml")]
    fn read_rules_from_file_with_yaml() {
        let result = Reader::read_rules_from_file(
            &Path::new(&format!("{}/tests/resources/rules/rules.yaml", env!("CARGO_MANIFEST_DIR")))
        );
        test_loaded_rules(result);
    }
}
