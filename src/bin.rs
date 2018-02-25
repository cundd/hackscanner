// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

extern crate error_chain;
extern crate simplelog;
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
    configure_logging(simplelog::LogLevelFilter::Trace);
    let rules = get_builtin_rules();
    let matches = file_finder::find_files(env::current_dir().unwrap(), &rules);
    for entry in matches {
        println!("{}", entry.path().display());
    }

    Ok(())
}


fn configure_logging(log_level_filter: simplelog::LogLevelFilter) {
    let mut loggers: Vec<Box<simplelog::SharedLogger>> = vec![];

    if let Some(core_logger) = simplelog::TermLogger::new(log_level_filter, simplelog::Config::default()) {
        loggers.push(core_logger);
    } else {
        loggers.push(simplelog::SimpleLogger::new(log_level_filter, simplelog::Config::default()));
    }

    match simplelog::CombinedLogger::init(loggers) {
        Ok(_) => (),
        Err(e) => panic!(e),
    }
}