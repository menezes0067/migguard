use crate::parser::Statement;
use crate::rule::{Finding, Rule, Severity, has_flag, normalize};

pub struct CreateIndexConcurrently;

impl Rule for CreateIndexConcurrently {
    fn check(&self, stmt: &Statement) -> Vec<Finding> {
        let sql = normalize(&stmt.sql);

        let search_flag = has_flag(&sql, "CONCURRENTLY");

        let is_create_index = 
            sql.starts_with("CREATE INDEX") || 
            sql.starts_with("CREATE UNIQUE INDEX");

        if is_create_index && !search_flag {
            return vec![Finding {
                rule: "create_index_concurrently",
                severity: Severity::Error,
                line: stmt.line,
                message: String::from("
                CREATE INDEX without CONCURRENTLY blocks writes to the table while the index is being created."),
                help: String::from("use CREATE INDEX CONCURRENTLY"),
            }];
        } 

        Vec::new()
    }
}