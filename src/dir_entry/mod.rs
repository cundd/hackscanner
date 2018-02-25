use std::fs;
use std::path::Path;
use std::ffi::OsStr;
use std::io;
use std::fmt::Debug;

pub trait DirEntryTrait: Debug {
    /// The full path that this entry represents.
    ///
    /// See [`walkdir::DirEntry::path`] for more details
    fn path(&self) -> &Path;

    /// Returns `true` if and only if this entry was created from a symbolic
    /// link. This is unaffected by the [`follow_links`] setting.
    ///
    /// See [`walkdir::DirEntry::path_is_symlink`] for more details
    fn path_is_symlink(&self) -> bool;

    /// Return the metadata for the file that this entry points to.
    ///
    /// See [`walkdir::DirEntry::metadata`] for more details
    fn metadata(&self) -> io::Result<fs::Metadata>;

    /// Return the file type for the file that this entry points to.
    ///
    /// See [`walkdir::DirEntry::file_type`] for more details
    fn file_type(&self) -> fs::FileType;

    /// Return the file name of this entry.
    ///
    /// See [`walkdir::DirEntry::file_name`] for more details
    fn file_name(&self) -> &OsStr;
}

pub mod walkdir;
pub mod standalone;

pub type WalkdirDirEntry = walkdir::DirEntry;
pub type StandaloneDirEntry = standalone::DirEntry;
