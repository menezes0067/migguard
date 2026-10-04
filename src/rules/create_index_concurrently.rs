use crate::parser::Statement;
use crate::rule::{Finding, Severity};

pub fn check(stmt: &Statement) -> Option<Finding> {
    let sql = stmt
        .sql
        .to_uppercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    let is_create_index = sql.starts_with("CREATE INDEX") || sql.starts_with("CREATE UNIQUE INDEX");

    if is_create_index && !sql.contains("CONCURRENTLY") {
        Some(Finding {
            rule: "create_index_concurrently",
            severity: Severity::Error,
            line: stmt.line,
            message: String::from("
            CREATE INDEX without CONCURRENTLY blocks writes to the table while the index is being created.",
        ),
            help: String::from("USE CREATE INDEX CONCURRENTLY"),
        })
    } else {
        None
    }
}
