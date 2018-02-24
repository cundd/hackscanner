// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

extern crate error_chain;
extern crate walkdir;
extern crate regex;
extern crate hackscanner_lib;

use std::env;
use hackscanner_lib::*;

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        use error_chain::ChainedError; // trait which holds `display_chain`
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "{}", e.display_chain()).expect(errmsg);
        ::std::process::exit(1);
    }
}

// Most functions will return the `Result` type, imported from the
// `errors` module. It is a typedef of the standard `Result` type
// for which the error type is always our own `Error`.
fn run() -> Result<(), Error> {
    let rules = vec![
        Rule::new(Some("tx_mocfilemanager".to_owned()), None, 8)
    ];
    let matches= file_finder::find_files(env::current_dir().unwrap(), &rules);
    for entry in matches {
        println!("{}", entry.path().display());
    }


    Ok(())
}
