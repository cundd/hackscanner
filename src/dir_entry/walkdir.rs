use std::fs;
use std::io;
use std::path::Path;
use std::ffi::OsStr;
use walkdir;
use fs::FileTypeTrait;
use super::DirEntryTrait;

#[derive(Debug, Clone)]
pub struct DirEntry {
    raw: walkdir::DirEntry,
}

impl DirEntry {
    pub fn from_dir_entry(raw: walkdir::DirEntry) -> Self {
        DirEntry { raw }
    }

    /// Return the depth at which this entry was created relative to the root.
    ///
    /// See [`walkdir::DirEntry::depth`] for more details
    #[allow(unused)]
    fn depth(&self) -> usize {
        self.raw.depth()
    }
}

impl DirEntryTrait for DirEntry {
    /// The full path that this entry represents.
    ///
    /// See [`walkdir::DirEntry::path`] for more details
    fn path(&self) -> &Path {
        &self.raw.path()
    }

    /// Return `true` if and only if this entry was created from a symbolic
    /// link. This is unaffected by the [`follow_links`] setting.
    ///
    /// See [`walkdir::DirEntry::path_is_symlink`] for more details
    fn path_is_symlink(&self) -> bool {
        self.raw.path_is_symlink()
    }

    /// Return the metadata for the file that this entry points to.
    ///
    /// See [`walkdir::DirEntry::metadata`] for more details
    fn metadata(&self) -> io::Result<fs::Metadata> {
        let walkdir_error = match self.raw.metadata() {
            Ok(m) => return Ok(m),
            Err(e) => e
        };

        match walkdir_error.into_io_error() {
            Some(e) => Err(e),
            None => Err(io::Error::new(io::ErrorKind::Other, "Loop error"))
        }
    }

    /// Return the file type for the file that this entry points to.
    ///
    /// See [`walkdir::DirEntry::file_type`] for more details
    fn file_type(&self) -> Box<FileTypeTrait> {
        Box::new(self.raw.file_type())
    }

    /// Return the file name of this entry.
    ///
    /// See [`walkdir::DirEntry::file_name`] for more details
    fn file_name(&self) -> &OsStr {
        self.raw.file_name()
    }
}
