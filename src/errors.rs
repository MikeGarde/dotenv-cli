use std::fmt;

/// User/input errors — caught in main, printed without stack trace, exits 1.
#[derive(Debug)]
pub struct RuleViolationError(pub String);

/// .env file parse errors — includes line number, exits 1.
#[derive(Debug)]
pub struct EnvParseError {
    pub line: usize,
    pub message: String,
}

impl fmt::Display for RuleViolationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for EnvParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error parsing .env file at line {}: {}",
            self.line, self.message
        )
    }
}

impl std::error::Error for RuleViolationError {}
impl std::error::Error for EnvParseError {}
