#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Severity {
    CRITICAL = 90,
    MAJOR = 60,
    MINOR = 40,
    NOTICE = 20,

    EASE = -20,
    WHITELIST = -250,
}
