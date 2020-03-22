pub use self::error::ContentClassificationError;
pub use self::error::ContentClassificationErrorKind;
use super::Classification;
use super::ClassifierTrait;
use super::Violation;
use crate::errors::Result;
use crate::matcher::Matcher;
use crate::rule::*;
use crate::DirEntryTrait;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

mod error;

/// Number of bytes to read from files
const BUFFER_SIZE: usize = 1024 * 1024 * 4;

pub struct ContentClassifier {
    file_content_cache: String,
    path: PathBuf,
}

impl ContentClassifier {
    fn get_file_content<'f, D: DirEntryTrait>(
        &mut self,
        entry: &D,
    ) -> Result<&str, ContentClassificationError> {
        if !(entry.path() == self.path.as_path()) {
            unreachable!(
                "Entry path does not match path stored in struct ContentClassifier. \n{:?} != \n{:?}",
                entry.path(),
                self.path
            );
        }
        if self.file_content_cache.is_empty() {
            self.read_file_content(entry)?;
        }

        Ok(self.file_content_cache.as_str())
    }

    fn read_file_content<D: DirEntryTrait>(
        &mut self,
        entry: &D,
    ) -> Result<(), ContentClassificationError> {
        let path = entry.path();
        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) => return Err(ContentClassificationError::from_io_error(path, e)),
        };

        trace!("Will read file {}", path.display());
        let mut buffer = Vec::with_capacity(BUFFER_SIZE);

        match file.take(BUFFER_SIZE as u64).read_to_end(&mut buffer) {
            Ok(_bytes_count) => {}
            Err(e) => {
                trace!("Could not read file '{}': {}", path.display(), e);
                return Err(ContentClassificationError::from_io_error(path, e));
            }
        };
        trace!("Did read file {}", path.display());

        self.file_content_cache = String::from_utf8_lossy(&buffer).to_string();

        Ok(())
    }
}

impl<'a, D: DirEntryTrait> ClassifierTrait<D> for ContentClassifier {
    fn new(entry: &D) -> Self {
        ContentClassifier {
            file_content_cache: "".to_owned(),
            path: entry.path().to_owned(),
        }
    }

    fn classify(&mut self, entry: &D, rule: &Rule) -> Classification {
        match self.get_file_content(entry) {
            Ok(s) => {
                if Matcher::match_entry_content(rule, s) {
                    trace!("Rule's content does match");

                    Classification::Match(Violation::from(rule))
                } else {
                    trace!("Rule's content does not match");

                    Classification::NoMatch
                }
            }
            // If the file content could not be read build a Violation from the error
            Err(e) => {
                let violation_option = Violation::with_rule_and_file_io_error(rule.clone(), &e);
                match violation_option {
                    Some(v) => Classification::Error(v),
                    None => Classification::NoMatch,
                }
            }
        }
    }
}
