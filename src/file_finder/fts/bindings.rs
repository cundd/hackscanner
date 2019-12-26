#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

#[cfg(target_os = "macos")]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/file_finder/fts/bindings_macos.rs"
));

#[cfg(target_os = "linux")]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/file_finder/fts/bindings_unknown_linux.rs"
));
