// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

extern crate ansi_term;
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate hackscanner_lib;
#[macro_use]
extern crate log;
extern crate regex;
extern crate simplelog;
extern crate term;
extern crate walkdir;

use clap::App;
use clap::Arg;
use clap::ArgMatches;
use hackscanner_lib::*;
use std::env;
use std::path::Path;
use simplelog::TerminalMode;

mod ui;

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
    let app = App::new("hackscanner")
        .version("0.2.0")
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
        .arg(Arg::with_name("min-severity")
            .short("m")
            .takes_value(true)
            .help("Sets the minimum severity to display (CRITICAL, MAJOR, MINOR, NOTICE, ALL)"))
        ;

    #[cfg(any(feature = "json", feature = "yaml"))]
        let app = app.arg(Arg::with_name("configuration")
        .help("File with additional rules")
        .short("c")
        .long("configuration")
        .takes_value(true)
    );

    let matches = app.get_matches();

    configure_logging(&matches).unwrap();

    #[cfg(not(any(feature = "json", feature = "yaml")))]
        let rules = &get_merged_rules(None)?;
    #[cfg(any(feature = "json", feature = "yaml"))]
        let rules = &get_merged_rules(match matches.value_of("configuration") {
        Some(c) => {
            info!("Load custom rules from '{}'", c);
            Some(Path::new(c))
        }
        None => None,
    })?;

    let min_severity = get_minimum_severity(&matches);
    let root = get_root(&matches);

    let files = file_finder::find_files(root, rules);
    let pattern_rules = PatternRule::from_rules_filtered(rules);
    let mut ratings = classifier::classify_entries(&files, &pattern_rules);

    ratings.sort_unstable_by(|rating_a, rating_b| rating_b.rating().cmp(&rating_a.rating()));
    for rating in ratings {
        if rating.rating() >= min_severity {
            ui::print_rating(&rating);
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

fn get_minimum_severity(matches: &ArgMatches) -> isize {
    let min_severity = matches.value_of("min-severity");
    if min_severity.is_none() {
        return Severity::NOTICE as isize;
    }

    match min_severity.unwrap().to_uppercase().as_ref() {
        "CRITICAL" => Severity::CRITICAL as isize,
        "MAJOR" => Severity::MAJOR as isize,
        "MINOR" => Severity::MINOR as isize,
        "NOTICE" => Severity::NOTICE as isize,
        _ => Severity::WHITELIST as isize
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
    let mut config = simplelog::Config::default();
    config.time_format = Some("%H:%M:%S%.3f");

    if let Some(core_logger) = simplelog::TermLogger::new(log_level_filter, config, TerminalMode::Mixed) {
        loggers.push(core_logger);
    } else {
        loggers.push(simplelog::SimpleLogger::new(log_level_filter, config));
    }

    match simplelog::CombinedLogger::init(loggers) {
        Ok(_) => Ok(()),
        Err(e) => bail!(format!("{}",e)),
    }
}