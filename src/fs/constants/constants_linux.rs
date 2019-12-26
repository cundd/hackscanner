use libc::c_int;

/// Values for the FLAG argument to the user function passed to `ftw' and 'nftw'.
/// Regular file.
#[allow(unused)]
pub const FTW_F: c_int = 0;
/// Directory.
#[allow(unused)]
pub const FTW_D: c_int = 1;
/// Unreadable directory.
#[allow(unused)]
pub const FTW_DNR: c_int = 2;
/// Unstatable file.
#[allow(unused)]
pub const FTW_NS: c_int = 3;
/// Symbolic link.
#[allow(unused)]
pub const FTW_SL: c_int = 4;

/// These flags are only passed from the `nftw' function.  */
/// Directory, all subdirs have been visited.
#[allow(unused)]
pub const FTW_DP: c_int = 5;
/// Symbolic link naming non-existing file.
#[allow(unused)]
pub const FTW_SLN: c_int = 6;

/// Perform physical walk, ignore symlinks.
#[allow(unused)]
pub const FTW_PHYS: c_int = 1;
/// Report only files on same file system as the argument.
#[allow(unused)]
pub const FTW_MOUNT: c_int = 2;
/// Change to current directory while processing it.
#[allow(unused)]
pub const FTW_CHDIR: c_int = 4;
/// Report files in directory before directory itself.
#[allow(unused)]
pub const FTW_DEPTH: c_int = 8;
