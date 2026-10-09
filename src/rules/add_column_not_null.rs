use crate::parser::Statement;
use crate::rule::{Finding, Rule, Severity, has_flag, normalize};

pub struct AddColumnNotNull;

impl Rule for AddColumnNotNull {
    fn check(&self, stmt: &Statement) -> Vec<Finding> {
        let sql = normalize(&stmt.sql);

        let is_alter_table = sql.starts_with("ALTER TABLE");
        let adds_column = has_flag(&sql, "ADD COLUMN");
        let is_not_null = has_flag(&sql, "NOT NULL");
        let has_default = has_flag(&sql, "DEFAULT");

        if is_alter_table && adds_column && is_not_null && !has_default {
            return vec![Finding {
                rule: "add-column-not-null",
                severity: Severity::Warn,
                line: stmt.line,
                message: String::from(
                    "ADD COLUMN ... NOT NULL without DEFAULT fails on tables that already have rows.",
                ),
                help: String::from(
                    "add the column as nullable, backfill, then add the constraint, or use a constant DEFAULT",
                ),
            }];
        }

        Vec::new()
    }
}
