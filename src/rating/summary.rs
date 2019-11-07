use crate::{Rating, Severity};

pub struct Summary {
    critical: isize,
    major: isize,
    minor: isize,
    notice: isize,
    clean: isize,
}

impl Summary {
    /// Build a summary of the overall Ratings
    pub fn build(ratings: &Vec<Rating>) -> Self {
        let mut summary = Summary {
            critical: 0,
            major: 0,
            minor: 0,
            notice: 0,
            clean: 0,
        };
        for rating in ratings {
            match rating.rating() {
                r if r >= Severity::CRITICAL as isize => summary.critical += 1,
                r if r >= Severity::MAJOR as isize => summary.major += 1,
                r if r >= Severity::MINOR as isize => summary.minor += 1,
                r if r >= Severity::NOTICE as isize => summary.notice += 1,
                _ => summary.clean += 1,
            }
        }

        summary
    }

    pub fn critical(&self) -> isize {
        self.critical
    }

    pub fn major(&self) -> isize {
        self.major
    }

    pub fn minor(&self) -> isize {
        self.minor
    }

    pub fn notice(&self) -> isize {
        self.notice
    }

    pub fn clean(&self) -> isize {
        self.clean
    }

    /// Return the number of ratings including and greater than the `severity`
    pub fn ratings_above(&self, severity: Severity) -> isize {
        match severity {
            Severity::CRITICAL => self.critical,
            Severity::MAJOR => self.critical + self.major,
            Severity::MINOR => self.critical + self.major + self.minor,
            Severity::NOTICE | _ => self.critical + self.major + self.minor + self.notice,
        }
    }
}
