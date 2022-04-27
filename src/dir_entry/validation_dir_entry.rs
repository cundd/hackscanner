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
    path_buf: PathBuf,
    file_type: StandaloneFileType,
}

impl DirEntry {
    pub fn from_path_str<P: Into<PathBuf>>(raw: P) -> Self {
        let path_buf = raw.into();
        let file_type = DirEntry::detect_file_type(&path_buf);

        DirEntry {
            path_buf,
            file_type,
        }
    }

    fn detect_file_type(path: &Path) -> StandaloneFileType {
        if let Ok(metadata) = path.metadata() {
            let file_type = metadata.file_type();

            StandaloneFileType::from_file_type(&file_type)
        } else {
            StandaloneFileType::Unknown
        }
    }
}

impl DirEntryTrait for DirEntry {
    /// The full path that this entry represents.
    ///
    /// See [`walkdir::DirEntry::path`] for more details
    fn path(&self) -> &Path {
        &self.path_buf
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
        self.path_buf.metadata()
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
        self.path_buf
            .file_name()
            .expect("An invalid DirEntry instance has been created. This must not have happened")
    }
}
