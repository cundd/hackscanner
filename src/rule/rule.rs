use super::RuleTrait;

/// "raw" Rule
#[derive(Debug, Clone)]
pub struct Rule {
    path: Option<String>,
    content: Option<String>,
    score: i8,
}

impl Rule {
    pub fn new(path: Option<String>, content: Option<String>, score: i8) -> Rule
    {
        Rule {
            path,
            content,
            score,
        }
    }
}

impl RuleTrait<String> for Rule {
    fn path(&self) -> Option<String> {
        self.path.clone()
    }

    fn content(&self) -> Option<String> {
        self.content.clone()
    }
    fn score(&self) -> i8 {
        self.score
    }
}
