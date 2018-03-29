use libc::c_int;

/// Valid flags for the 3rd argument to the function that is passed as the
/// second argument to ftw(3) and nftw(3).  Say it three times fast!

/// File.
#[allow(unused)]
pub const FTW_F: c_int = 0;
/// Directory.
#[allow(unused)]
pub const FTW_D: c_int = 1;
/// Directory without read permission.
#[allow(unused)]
pub const FTW_DNR: c_int = 2;
/// Directory with subdirectories visited.
#[allow(unused)]
pub const FTW_DP: c_int = 3;
/// Unknown type; stat() failed.
#[allow(unused)]
pub const FTW_NS: c_int = 4;
/// Symbolic link.
#[allow(unused)]
pub const FTW_SL: c_int = 5;
/// Sym link that names a nonexistent file.
#[allow(unused)]
pub const FTW_SLN: c_int = 6;

/// Flags for use as the 4th argument to nftw(3).  These may be ORed together.

/// Physical walk, don't follow sym links.
#[allow(unused)]
pub const FTW_PHYS: c_int = 0x01;
/// The walk does not cross a mount point.
#[allow(unused)]
pub const FTW_MOUNT: c_int = 0x02;
/// Subdirs visited before the dir itself.
#[allow(unused)]
pub const FTW_DEPTH: c_int = 0x04;
/// Change to a directory before reading it.
#[allow(unused)]
pub const FTW_CHDIR: c_int = 0x08;