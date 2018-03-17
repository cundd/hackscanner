use libc::c_int;
use super::FileTypeTrait;
use std::fs;
use super::super::constants::*;

#[derive(Debug, Clone)]
pub enum FileType {
    File,
    Directory,
    Symlink,
    Unknown,
}

impl FileType {
    pub fn from_ftw(ftw: c_int) -> Self {
        match ftw {
            FTW_F => FileType::File,
            FTW_D => FileType::File,
            FTW_DNR => FileType::Directory,
            FTW_DP => FileType::Directory,
            FTW_SL => FileType::Symlink,
            FTW_SLN => FileType::Symlink,
            FTW_NS => FileType::Unknown,
            _ => FileType::Unknown
        }
    }

    pub fn from_file_type(file_type: &fs::FileType) -> Self {
        if file_type.is_file() {
            FileType::File
        } else if file_type.is_dir() {
            FileType::Directory
        } else if file_type.is_symlink() {
            FileType::Symlink
        } else {
            FileType::Unknown
        }
    }
}

impl FileTypeTrait for FileType {
    fn is_dir(&self) -> bool {
        match self {
            &FileType::File => false,
            &FileType::Directory => true,
            &FileType::Symlink => false,
            &FileType::Unknown => false,
        }
    }

    fn is_file(&self) -> bool {
        match self {
            &FileType::File => true,
            &FileType::Directory => false,
            &FileType::Symlink => false,
            &FileType::Unknown => false,
        }
    }

    fn is_symlink(&self) -> bool {
        match self {
            &FileType::File => false,
            &FileType::Directory => false,
            &FileType::Symlink => true,
            &FileType::Unknown => false,
        }
    }
}