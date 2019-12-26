use std::error::Error as StdError;
use std::fmt::{Display, Error as FmtError, Formatter};
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct ContentClassificationError {
    inner: Box<dyn StdError>,
    kind: ContentClassificationErrorKind,
    path: PathBuf,
}

#[doc(hidden)]
impl ContentClassificationError {
    pub fn from_io_error(path: &Path, error: IoError) -> Self {
        let kind = ContentClassificationErrorKind::from(error.kind());
        ContentClassificationError {
            inner: Box::new(error),
            kind,
            path: path.to_path_buf(),
        }
    }

    pub fn kind(&self) -> ContentClassificationErrorKind {
        self.kind
    }

    pub fn long_description(&self) -> String {
        format!("{}: {}", self, self.inner)
    }
}

impl Display for ContentClassificationError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self.kind() {
            ContentClassificationErrorKind::NotExists => {
                write!(f, "File {:?} does not exist", self.path)
            }
            ContentClassificationErrorKind::NotReadable => {
                write!(f, "Could not read file {:?}", self.path)
            }
            ContentClassificationErrorKind::Unknown => {
                write!(f, "Could not read file {:?}", self.path)
            }
        }
    }
}

impl StdError for ContentClassificationError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.inner.as_ref())
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum ContentClassificationErrorKind {
    NotExists,
    NotReadable,
    Unknown,
}

impl From<IoErrorKind> for ContentClassificationErrorKind {
    fn from(e: IoErrorKind) -> Self {
        match e {
            IoErrorKind::NotFound => ContentClassificationErrorKind::NotExists,
            IoErrorKind::PermissionDenied => ContentClassificationErrorKind::NotReadable,
            _ => ContentClassificationErrorKind::Unknown,
        }
    }
}
