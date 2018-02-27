// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate simplelog;
extern crate walkdir;
extern crate regex;
extern crate clap;
extern crate hackscanner_lib;

use std::env;
use clap::Arg;
use clap::App;
use clap::ArgMatches;
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
    let matches = App::new("hackscanner")
        .version("0.1.0")
        .author("Daniel Corn <info@cundd.net>")
        .about("Scan the filesystem for hacked files")
        .arg(Arg::with_name("directory")
            .help("Search in this directory")
            .takes_value(true)
            .index(1))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity (-v = Info, -vv = Debug, -vvv = Trace)"))
        .get_matches();

    configure_logging(&matches).unwrap();

    let rules = &get_builtin_rules();

    let root = get_root(&matches);

    let files = file_finder::find_files(root, rules);
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

fn get_root(matches: &ArgMatches) -> String {
    match matches.value_of("directory") {
        Some(d) => d.to_owned(),
        None => String::from(env::current_dir().unwrap().to_string_lossy()),
    }
}


fn configure_logging(matches: &ArgMatches) -> Result<(), Error> {
    let log_level_filter = match matches.occurrences_of("v") {
        1 => simplelog::LevelFilter::Info,
        2 => simplelog::LevelFilter::Debug,
        3 => simplelog::LevelFilter::Trace,
        _ => simplelog::LevelFilter::Warn,
    };

    let mut loggers: Vec<Box<simplelog::SharedLogger>> = vec![];

    if let Some(core_logger) = simplelog::TermLogger::new(log_level_filter, simplelog::Config::default()) {
        loggers.push(core_logger);
    } else {
        loggers.push(simplelog::SimpleLogger::new(log_level_filter, simplelog::Config::default()));
    }

    match simplelog::CombinedLogger::init(loggers) {
        Ok(_) => Ok(()),
        Err(e) => bail!(format!("{}",e)),
    }
}