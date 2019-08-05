use ansi_term::Colour;
use term;
use hackscanner_lib::*;

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
    let rating_description = if rating.rating() >= Severity::CRITICAL as isize {
        Colour::RGB(225, 17, 0).paint("[CRITICAL]")
    } else if rating.rating() >= Severity::MAJOR as isize {
        Colour::RGB(237, 131, 0).paint("[MAJOR]   ")
    } else if rating.rating() >= Severity::MINOR as isize {
        Colour::RGB(245, 207, 0).paint("[MINOR]   ")
    } else if rating.rating() >= Severity::NOTICE as isize {
        Colour::RGB(255, 255, 0).paint("[NOTICE]  ")
    } else {
        Colour::Blue.paint("[CLEAN]   ")
    };

    println!(
        "{} {} \t(Rules: {})",
        rating_description,
        Colour::Black.bold().paint(get_path_as_string(rating)),
        join_rules(rating.rules())
    );
}

fn print_rating_simple(rating: &Rating) {
    let rating_description = if rating.rating() >= Severity::CRITICAL as isize {
        "[CRITICAL]"
    } else if rating.rating() >= Severity::MAJOR as isize {
        "[MAJOR]   "
    } else if rating.rating() >= Severity::MINOR as isize {
        "[MINOR]   "
    } else if rating.rating() >= Severity::NOTICE as isize {
        "[NOTICE]  "
    } else {
        "[CLEAN]   "
    };

    println!(
        "{} {} \t(Rules: {})",
        rating_description,
        get_path_as_string(rating),
        join_rules(rating.rules())
    );
}

fn join_rules(rules: &Vec<Rule>) -> String {
    rules.iter().fold(
        String::new(),
        |acc, rule| {
            let separator = if !acc.is_empty() {
                ", "
            } else {
                ""
            };

            acc + separator + &format!("{}", rule.name())
        },
    )
}
