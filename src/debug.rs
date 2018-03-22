// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

//#[macro_use]
//extern crate error_chain;
//extern crate simplelog;
extern crate hackscanner_lib;

use std::thread;
use std::thread::JoinHandle;

use hackscanner_lib::*;
use self::file_finder::*;
use self::file_finder::fts::FileFinder;


fn main() {
    let rules = vec![Rule::new("2".to_string(), Severity::NOTICE, Some("\\.tx_mocfilemanager".to_owned()), None)];
    let mut handles: Vec<JoinHandle<_>> = vec![];

    for _ in 0..4 {
        let rules = rules.clone();
        let handle: JoinHandle<_> = thread::spawn(move || {
            FileFinder::new().find(env!("CARGO_MANIFEST_DIR"), &rules);
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }
}
