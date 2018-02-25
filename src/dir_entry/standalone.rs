use std::ffi::OsStr;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::fs;
use std::fs::FileType;
use super::DirEntryTrait;

#[derive(Debug)]
pub struct DirEntry {
    raw: PathBuf,
    #[allow(unused)]
    file_type: FileType,
}

impl DirEntry {
    pub fn from_path(raw: PathBuf) -> Result<Self, io::Error> {
        let metadata = raw.metadata()?;
        let file_type = metadata.file_type();

        Ok(DirEntry { raw, file_type })
    }
}

impl DirEntryTrait for DirEntry {
    /// The full path that this entry represents.
    ///
    /// See [`walkdir::DirEntry::path`] for more details
    fn path(&self) -> &Path {
        &self.raw
    }

    /// Returns `true` if and only if this entry was created from a symbolic
    /// link. This is unaffected by the [`follow_links`] setting.
    ///
    /// See [`walkdir::DirEntry::path_is_symlink`] for more details
    fn path_is_symlink(&self) -> bool {
        if let Ok(m) = self.metadata() {
            m.file_type().is_symlink()
        } else {
            false
        }
    }

    /// Return the metadata for the file that this entry points to.
    ///
    /// See [`walkdir::DirEntry::metadata`] for more details
    fn metadata(&self) -> io::Result<fs::Metadata> {
        self.raw.metadata()
    }

    /// Return the file type for the file that this entry points to.
    ///
    /// See [`walkdir::DirEntry::file_type`] for more details
    fn file_type(&self) -> fs::FileType {
        self.metadata()
            .expect("An invalid DirEntry instance has been created. This must not have happened")
            .file_type()
    }

    /// Return the file name of this entry.
    ///
    /// See [`walkdir::DirEntry::file_name`] for more details
    fn file_name(&self) -> &OsStr {
        self.raw.file_name()
            .expect("An invalid DirEntry instance has been created. This must not have happened")
    }
}
