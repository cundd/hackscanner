use super::Violation;
use std::fmt::{Formatter, Error, Debug};

pub enum Classification {
    Empty,
    NotApplicable,
    NoMatch,
    Match(Violation),
    Error(Violation),
}

impl Debug for Classification {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", match self {
            Classification::NotApplicable => "NotApplicable",
            Classification::Empty => "Empty",
            Classification::NoMatch => "NoMatch",
            Classification::Match(_) => "Match",
            Classification::Error(_) => "Error",
        })
    }
}
