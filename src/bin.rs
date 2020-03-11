// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate log;

use clap::App;
use clap::Arg;
use clap::ArgMatches;
use hackscanner_lib::*;
use simplelog;
use simplelog::TerminalMode;
use std::env;
use std::path::Path;

mod ui;

fn main() {
    if let Err(ref e) = run() {
        use error_chain::ChainedError;
        use std::io::Write; // trait which holds `display_chain`
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "{}", e.display_chain()).expect(errmsg);
        ::std::process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let app = App::new("hackscanner")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Daniel Corn <info@cundd.net>")
        .about("Scan the filesystem for hacked files")
        .arg(Arg::with_name("directory")
            .help("Search in this directory")
            .takes_value(true)
            .index(1))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help(get_verbosity_help()))
        .arg(Arg::with_name("min-severity")
            .short("m")
            .takes_value(true)
            .help("Sets the minimum severity to display (CRITICAL, MAJOR, MINOR, NOTICE, ALL)"))
        .arg(Arg::with_name("quiet")
            .short("q")
            .long("quiet")
            .alias("silent")
            .help("Do not write to standard output if no violations > min-severity are found"))
        .arg(Arg::with_name("validate")
            .short("l")
            .long("validate")
            .takes_value(true)
            .value_name("test-path")
            .help("Check if the given test-path would create a violation (ignores if the path exists)"))
        ;

    #[cfg(any(feature = "json", feature = "yaml"))]
        let app = app.arg(
        Arg::with_name("configuration")
            .help("File with additional rules")
            .short("c")
            .long("configuration")
            .takes_value(true),
    );

    let matches = app.get_matches();

    configure_logging(&matches).unwrap();

    #[cfg(not(any(feature = "json", feature = "yaml")))]
        let rules = &get_merged_rules(None)?;
    #[cfg(any(feature = "json", feature = "yaml"))]
        let rules = &get_merged_rules(match matches.value_of("configuration") {
        Some(c) => Some(Path::new(c)),
        None => None,
    })?;

    let pattern_rules = PatternRule::from_rules_filtered(rules);

    match matches.value_of("validate") {
        Some(test_path) => validate(&matches, pattern_rules, test_path),
        None => scan(&matches, rules, pattern_rules),
    }
}


// Trace is only supported on debug-builds
#[cfg(debug_assertions)]
fn get_verbosity_help() -> &'static str {
    "Sets the level of verbosity (-v = Info, -vv = Debug, -vvv = Trace)"
}

#[cfg(not(debug_assertions))]
fn get_verbosity_help() -> &'static str {
    "Sets the level of verbosity (-v = Info, -vv = Debug)"
}

fn scan(
    matches: &ArgMatches,
    rules: &Vec<Rule>,
    pattern_rules: Vec<PatternRule>,
) -> Result<(), Error> {
    let min_severity = get_minimum_severity(&matches);
    let root = get_root(&matches);
    let quiet = matches.is_present("quiet");

    let files = file_finder::find_files(root, rules);

    let ratings = sort_ratings(&rating::rate_entries(&files, &pattern_rules));
    let summary = Summary::build(&ratings);
    if !quiet || 0 < summary.ratings_above(min_severity) {
        ui::print_summary(min_severity, &summary);
        ui::print_ratings(min_severity, &ratings);
    }

    Ok(())
}

fn validate(
    matches: &ArgMatches,
    pattern_rules: Vec<PatternRule>,
    test_path: &str,
) -> Result<(), Error> {
    let entry = ValidationDirEntry::from_path_str(test_path);
    if !entry.path().exists() {
        bail!(format!("File {} does not exist", entry.path().display()))
    }

    let rating = rate_entry(&entry, &pattern_rules);
    ui::print_validation(&rating, matches.occurrences_of("v") > 0);

    Ok(())
}

fn get_root(matches: &ArgMatches<'_>) -> String {
    match matches.value_of("directory") {
        Some(d) => d.to_owned(),
        None => String::from(env::current_dir().unwrap().to_string_lossy()),
    }
}

fn get_minimum_severity(matches: &ArgMatches<'_>) -> Severity {
    let min_severity = matches.value_of("min-severity");
    if min_severity.is_none() {
        return Severity::NOTICE;
    }

    match min_severity.unwrap().to_uppercase().as_ref() {
        "CRITICAL" => Severity::CRITICAL,
        "MAJOR" => Severity::MAJOR,
        "MINOR" => Severity::MINOR,
        "NOTICE" => Severity::NOTICE,
        _ => Severity::WHITELIST,
    }
}

fn configure_logging(matches: &ArgMatches<'_>) -> Result<(), Error> {
    let log_level_filter = match matches.occurrences_of("v") {
        1 => simplelog::LevelFilter::Info,
        2 => simplelog::LevelFilter::Debug,
        3 => simplelog::LevelFilter::Trace,
        _ => simplelog::LevelFilter::Warn,
    };

    let mut loggers: Vec<Box<dyn simplelog::SharedLogger>> = vec![];
    let mut config = simplelog::Config::default();
    config.time_format = Some("%H:%M:%S%.3f");

    if let Some(core_logger) =
    simplelog::TermLogger::new(log_level_filter, config, TerminalMode::Mixed)
    {
        loggers.push(core_logger);
    } else {
        loggers.push(simplelog::SimpleLogger::new(log_level_filter, config));
    }

    match simplelog::CombinedLogger::init(loggers) {
        Ok(_) => Ok(()),
        Err(e) => bail!(format!("{}", e)),
    }
}
