use regex::Regex;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use std::fmt::Result;

#[derive(Debug, Clone)]
pub enum RulePath {
    Regex(Regex),
    String(String),
}

impl RulePath {
    pub fn regex(&self) -> &Regex {
        match self {
            Self::Regex(r) => &r,
            Self::String(s) => panic!("Not a Regex Rule Path"),
        }
    }
    pub fn path_str(&self) -> &str {
        match self {
            Self::String(s) => &s,
            Self::Regex(r) => panic!("Not a String Rule Path"),
        }
    }
}

impl Display for RulePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", match self {
            RulePath::Regex(regex) => regex.to_string(),
            RulePath::String(string) => string.to_string(),
        })
    }
}
