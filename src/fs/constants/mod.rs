#[cfg(target_os = "macos")]
mod constants_macos;

#[cfg(target_os = "macos")]
pub use self::constants_macos::*;

#[cfg(target_os = "linux")]
mod constants_linux;

#[cfg(target_os = "linux")]
pub use self::constants_linux::*;

#[cfg(any(target_os = "freebsd", target_os = "openbsd", target_os = "netbsed"))]
mod constants_unix;

#[cfg(any(target_os = "freebsd", target_os = "openbsd", target_os = "netbsed"))]
pub use self::constants_unix::*;
