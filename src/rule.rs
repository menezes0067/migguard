use std::fmt;

use crate::parser::Statement;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Severity {
    Warn,
    Error,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Warn => write!(f, "warning"),
            Self::Error => write!(f, "error"),
        }
    }
}

#[derive(Debug)]
pub struct Finding {
    pub rule: &'static str,
    pub severity: Severity,
    pub line: usize,
    pub message: String,
    pub help: String,
}

impl fmt::Display for Finding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} [{}] {}\n   dica: {}\n",
            self.line, self.severity, self.rule, self.message, self.help
        )
    }
}

pub trait Rule{
    fn check(&self, stmt: &Statement) -> Vec<Finding>;
}

pub fn normalize(sql: &str) -> String {
    sql.to_uppercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}