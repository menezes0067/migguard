use sqlparser::ast::Statement;

use crate::parser::{ParsedStatement};
use crate::rule::{Finding, Rule, Severity};

pub struct CreateIndexConcurrently;

impl Rule for CreateIndexConcurrently {
    fn check(&self, stmt: &ParsedStatement) -> Vec<Finding> {
        let Statement::CreateIndex(index) = &stmt.ast else {
            return Vec::new();
        };

        if index.concurrently {
            return Vec::new();
        }

        vec![Finding {
            rule: "create-index-concurrently",
            severity: Severity::Error,
            line: stmt.line,
            message: String::from(
                "CREATE INDEX without CONCURRENTLY blocks writes on the table while the index is built",
            ),
            help: String::from("use CREATE INDEX CONCURRENTLY"),
        }]
    }
}
