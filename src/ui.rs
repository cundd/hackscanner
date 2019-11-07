use ansi_term::Colour;
use term;
use hackscanner_lib::*;

pub fn print_summary(min_severity: Severity, summary: &Summary) {
//    let summary = Summary::build(ratings);

    println!("[SUMMARY]");
    println!(
        "Detected {} violations with severity '{}' or higher",
        summary.ratings_above(min_severity),
        min_severity.description().to_lowercase()
    );
    let supports_color = match term::stdout() {
        Some(t) => t.supports_color(),
        None => false,
    };

    if supports_color {
        print_summary_colored(&summary);
    } else {
        print_summary_simple(&summary);
    }

    println!()
}

pub fn print_ratings(min_severity: Severity, ratings: &Vec<Rating>) {
    for rating in ratings {
        if rating.rating() >= min_severity as isize {
            print_rating(&rating);
        }
    }
}

pub fn print_rating(rating: &Rating) {
    let supports_color = match term::stdout() {
        Some(t) => t.supports_color(),
        None => false,
    };
    if supports_color {
        print_rating_colored(rating)
    } else {
        print_rating_simple(rating)
    }
}

fn get_path_as_string(rating: &Rating) -> String {
    rating.entry().path().to_string_lossy().into_owned()
}

fn print_rating_colored(rating: &Rating) {
    println!(
        "{} {} \t(Rules: {})",
        colored_description_for_severity(rating.rating().into()),
        Colour::Black.bold().paint(get_path_as_string(rating)),
        join_violations(rating.violations())
    );
}

fn print_rating_simple(rating: &Rating) {
    println!(
        "{} {} \t(Rules: {})",
        description_for_severity(rating.rating().into(), true),
        get_path_as_string(rating),
        join_violations(rating.violations())
    );
}

fn colored_description_for_severity(severity: Severity) -> String {
    format!("{}", color_for_severity(severity).paint(description_for_severity(severity, true)))
}

fn color_for_severity(severity: Severity) -> Colour {
    match severity {
        Severity::CRITICAL => Colour::RGB(225, 17, 0),
        Severity::MAJOR => Colour::RGB(237, 131, 0),
        Severity::MINOR => Colour::RGB(245, 207, 0),
        Severity::NOTICE => Colour::RGB(255, 255, 0),
        _ => Colour::Blue,
    }
}

fn description_for_severity(severity: Severity, brackets: bool) -> String {
    if brackets {
        format!("{:width$}", format!("[{}]", severity.description()), width = 10)
    } else {
        format!("{:width$}", format!("{}:", severity.description()), width = 10)
    }
}

fn print_summary_colored(summary: &Summary) {
    if summary.critical() > 0 {
        println!("{}", color_for_severity(Severity::CRITICAL)
            .paint(format!("{} {}", description_for_severity(Severity::CRITICAL, false), summary.critical())));
    }
    if summary.major() > 0 {
        println!("{}", color_for_severity(Severity::MAJOR)
            .paint(format!("{} {}", description_for_severity(Severity::MAJOR, false), summary.major())));
    }
    if summary.minor() > 0 {
        println!("{}", color_for_severity(Severity::MINOR)
            .paint(format!("{} {}", description_for_severity(Severity::MINOR, false), summary.minor())));
    }
    if summary.notice() > 0 {
        println!("{}", color_for_severity(Severity::NOTICE)
            .paint(format!("{} {}", description_for_severity(Severity::NOTICE, false), summary.notice())));
    }
    if summary.clean() > 0 {
        println!("{}", color_for_severity(Severity::NONE)
            .paint(format!("{} {}", description_for_severity(Severity::NONE, false), summary.clean())));
    }
}

fn print_summary_simple(summary: &Summary) {
    if summary.critical() > 0 {
        println!("{} {}", description_for_severity(Severity::CRITICAL, false), summary.critical());
    }
    if summary.major() > 0 {
        println!("{} {}", description_for_severity(Severity::MAJOR, false), summary.major());
    }
    if summary.minor() > 0 {
        println!("{} {}", description_for_severity(Severity::MINOR, false), summary.minor());
    }
    if summary.notice() > 0 {
        println!("{} {}", description_for_severity(Severity::NOTICE, false), summary.notice());
    }
    if summary.clean() > 0 {
        println!("{} {}", description_for_severity(Severity::NONE, false), summary.clean());
    }
}
