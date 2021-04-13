use crate::errors::Error;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Deserialize)]
pub enum Severity {
    CRITICAL = 90,
    MAJOR = 60,
    MINOR = 40,
    NOTICE = 20,

    NONE = 0,

    EASE = -20,
    EASE2X = -40,
    WHITELIST = -250,
}

impl Severity {
    pub fn description(&self) -> &str {
        match self {
            &Severity::CRITICAL => "CRITICAL",
            &Severity::MAJOR => "MAJOR",
            &Severity::MINOR => "MINOR",
            &Severity::NOTICE => "NOTICE",

            &Severity::NONE => "NONE",

            &Severity::EASE => "EASE",
            &Severity::EASE2X => "EASE 2X",
            &Severity::WHITELIST => "WHITELIST",
        }
    }
}

impl Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl FromStr for Severity {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "CRITICAL" => Ok(Severity::CRITICAL),
            "MAJOR" => Ok(Severity::MAJOR),
            "MINOR" => Ok(Severity::MINOR),
            "NOTICE" => Ok(Severity::NOTICE),
            "EASE" => Ok(Severity::EASE),
            "EASE2X" => Ok(Severity::EASE2X),
            "NONE" => Ok(Severity::NONE),
            "WHITELIST" => Ok(Severity::WHITELIST),
            _ => Err(s.into()),
        }
    }
}

impl From<isize> for Severity {
    fn from(rating: isize) -> Self {
        if rating >= Severity::CRITICAL as isize {
            Severity::CRITICAL
        } else if rating >= Severity::MAJOR as isize {
            Severity::MAJOR
        } else if rating >= Severity::MINOR as isize {
            Severity::MINOR
        } else if rating >= Severity::NOTICE as isize {
            Severity::NOTICE
        } else if rating == Severity::EASE as isize {
            Severity::EASE
        } else if rating == Severity::EASE2X as isize {
            Severity::EASE2X
        } else if rating == Severity::WHITELIST as isize {
            Severity::WHITELIST
        } else {
            Severity::NONE
        }
    }
}
