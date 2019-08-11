use DirEntryTrait;
use super::Classification;
use super::ClassifierTrait;
use super::Violation;
use rule::*;
use matcher::Matcher;
use errors::Result;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

/// Number of bytes to read from files
const BUFFER_SIZE: usize = 1024 * 4;

pub struct ContentClassifier {
    file_content_cache: String,
    path: PathBuf,
}

impl ContentClassifier {
    fn get_file_content<'f, D: DirEntryTrait>(&mut self, entry: &D) -> Result<String> {
        if !(entry.path() == self.path.as_path()) {
            panic!(
                "Entry path does not match path stored in struct ContentClassifier. \n{:?} != \n{:?}",
                entry.path(),
                self.path
            );
        }
        if self.file_content_cache.is_empty() {
            self.file_content_cache.push_str(&read_entry_content(entry)?);
        }

        Ok(self.file_content_cache.to_owned())
    }
}

impl<'a, D: DirEntryTrait> ClassifierTrait<D> for ContentClassifier {
    fn new(entry: &D) -> Self {
        ContentClassifier {
            file_content_cache: "".to_owned(),
            path: entry.path().to_owned(),
        }
    }

    fn classify(&mut self, entry: &D, rule: &PatternRule) -> Classification {
        match self.get_file_content(entry) {
            Ok(s) => {
                if !Matcher::match_entry_content(rule, &s) {
                    Classification::NoMatch
                } else {
                    Classification::Match(Violation::from(rule))
                }
            }
            // If the file content could not be read build a Violation from the error
            Err(e) => Classification::Error(Violation::from(&e))
        }
    }
}

fn read_entry_content<D: DirEntryTrait>(entry: &D) -> Result<String> {
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
