use super::DirEntryTrait;
use crate::fs::FileTypeTrait;
use crate::fs::StandaloneFileType;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct DirEntry {
    raw: PathBuf,
    file_type: StandaloneFileType,
}

impl DirEntry {
    pub fn from_path<P: Into<PathBuf>>(raw: P) -> Result<Self, io::Error> {
        let path_buf = raw.into();
        let metadata = path_buf.metadata()?;
        let file_type = metadata.file_type();

        Ok(Self::from_path_with_file_type(
            path_buf,
            StandaloneFileType::from_file_type(&file_type),
        ))
    }

    pub fn from_path_with_file_type<P: Into<PathBuf>>(
        raw: P,
        file_type: StandaloneFileType,
    ) -> Self {
        DirEntry {
            raw: raw.into(),
            file_type,
        }
    }
}

impl DirEntryTrait for DirEntry {
    /// The full path that this entry represents.
    ///
    /// See [`walkdir::DirEntry::path`] for more details
    fn path(&self) -> &Path {
        &self.raw
    }

    /// Return `true` if and only if this entry was created from a symbolic
    /// link. This is unaffected by the [`follow_links`] setting.
    ///
    /// See [`walkdir::DirEntry::path_is_symlink`] for more details
    fn path_is_symlink(&self) -> bool {
        self.file_type.is_symlink()
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
    fn file_type(&self) -> Box<dyn FileTypeTrait> {
        Box::new(self.file_type.clone())
    }

    /// Return the file name of this entry.
    ///
    /// See [`walkdir::DirEntry::file_name`] for more details
    fn file_name(&self) -> &OsStr {
        self.raw
            .file_name()
            .expect("An invalid DirEntry instance has been created. This must not have happened")
    }
}
