use std::fmt;
use dir_entry::DirEntryTrait;
use severity::Severity;

#[derive(Debug)]
pub struct Rating<'a> {
    entry: &'a DirEntryTrait,
    rating: isize,
}

impl<'a> Rating<'a> {
    pub fn new(entry: &'a DirEntryTrait, rating: isize) -> Self {
        Rating {
            entry,
            rating,
        }
    }

    pub fn entry(&self) -> &DirEntryTrait {
        self.entry
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

        format!("[{}] {}", self.rating_description(), path_as_string)
    }
}

impl<'a> fmt::Display for Rating<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
