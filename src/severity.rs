use std::fmt;
use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Deserialize)]
pub enum Severity {
    CRITICAL = 90,
    MAJOR = 60,
    MINOR = 40,
    NOTICE = 20,

    EASE = -20,
    WHITELIST = -250,
}

impl Severity {
    pub fn description(&self) -> String {
        match self {
            &Severity::CRITICAL => "CRITICAL".to_owned(),
            &Severity::MAJOR => "MAJOR".to_owned(),
            &Severity::MINOR => "MINOR".to_owned(),
            &Severity::NOTICE => "NOTICE".to_owned(),

            &Severity::EASE => "EASE".to_owned(),
            &Severity::WHITELIST => "WHITELIST".to_owned(),
        }
    }
}

impl Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
