use regex::Regex;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum RulePath {
    Regex(Regex),
    String(String),
}

impl RulePath {
    /// Return the path as regular expression
    ///
    /// # Panics
    ///
    /// Panics if this variant is not a `RulePath::Regex`
    pub fn regex(&self) -> &Regex {
        match self {
            Self::Regex(r) => &r,
            Self::String(_) => panic!("Not a Regex Rule Path"),
        }
    }

    /// Return the path as string
    ///
    /// # Panics
    ///
    /// Panics if this variant is not a `RulePath::String`
    pub fn path_str(&self) -> &str {
        match self {
            Self::String(s) => &s,
            Self::Regex(_) => panic!("Not a String Rule Path"),
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
