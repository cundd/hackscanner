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
    configure_logging(simplelog::LevelFilter::Trace);
    let rules = &get_builtin_rules();
    let files = file_finder::find_files(env::current_dir().unwrap(), rules);
    let pattern_rules = PatternRule::from_rules_filtered(rules);
    let mut ratings = classifier::classify_entries(&files, &pattern_rules);

    ratings.sort_unstable_by(|rating_a, rating_b| rating_b.rating().cmp(&rating_a.rating()));
    for rating in ratings {
        if rating.rating() > Severity::NOTICE as isize {
            println!("{}", rating);
        }
    }

    Ok(())
}


fn configure_logging(log_level_filter: simplelog::LevelFilter) {
    let mut loggers: Vec<Box<simplelog::SharedLogger>> = vec![];
    let mut config = simplelog::Config::default();
    config.time_format = Some("%H:%M:%S.%f");

    if let Some(core_logger) = simplelog::TermLogger::new(log_level_filter, config) {
        loggers.push(core_logger);
    } else {
        loggers.push(simplelog::SimpleLogger::new(log_level_filter, config));
    }

    match simplelog::CombinedLogger::init(loggers) {
        Ok(_) => (),
        Err(e) => panic!(e),
    }
}