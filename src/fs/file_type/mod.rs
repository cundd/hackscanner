mod standalone_file_type;

use std::fs;
pub use self::standalone_file_type::FileType as StandaloneFileType;


pub trait FileTypeTrait {
    /// Test whether this file type represents a directory.
    fn is_dir(&self) -> bool;

    /// Test whether this file type represents a regular file.
    fn is_file(&self) -> bool;

    /// Test whether this file type represents a symbolic link.
    ///
    /// The underlying [`Metadata`] struct needs to be retrieved
    /// with the [`fs::symlink_metadata`] function and not the
    /// [`fs::metadata`] function. The [`fs::metadata`] function
    /// follows symbolic links, so [`is_symlink`] would always
    /// return false for the target file.
    ///
    /// [`Metadata`]: struct.Metadata.html
    /// [`fs::metadata`]: fn.metadata.html
    /// [`fs::symlink_metadata`]: fn.symlink_metadata.html
    /// [`is_symlink`]: struct.FileType.html#method.is_symlink
    fn is_symlink(&self) -> bool;
}

impl FileTypeTrait for fs::FileType {
    fn is_dir(&self) -> bool {
        fs::FileType::is_dir(self)
    }

    fn is_file(&self) -> bool {
        fs::FileType::is_file(self)
    }

    fn is_symlink(&self) -> bool {
        fs::FileType::is_symlink(self)
    }
}
