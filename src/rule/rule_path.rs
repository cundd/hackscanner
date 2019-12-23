use regex::Regex;

pub enum RulePath {
    Regex(Regex),
    String(String),
}
