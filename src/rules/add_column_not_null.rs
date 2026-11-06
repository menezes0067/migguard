use sqlparser::ast::{AlterTableOperation, ColumnOption, Statement};

use crate::parser::ParsedStatement;
use crate::rule::{Finding, Rule, Severity};

pub struct AddColumnNotNull;

impl Rule for AddColumnNotNull {
    fn check(&self, stmt: &ParsedStatement) -> Vec<Finding> {
        let Statement::AlterTable(alter) = &stmt.ast else {
            return Vec::new();
        };

        let mut findings = Vec::new();

        for operation in &alter.operations {
            let AlterTableOperation::AddColumn { column_def, .. } = operation else {
                continue;
            };

            let is_not_null = column_def
                .options
                .iter()
                .any(|opt| matches!(opt.option, ColumnOption::NotNull));

            let has_default = column_def
                .options
                .iter()
                .any(|opt| matches!(opt.option, ColumnOption::Default(_)));

            if is_not_null && !has_default {
                findings.push(Finding {
                    rule: "add-column-not-null",
                    severity: Severity::Warn,
                    line: stmt.line,
                    message: format!(
                        "ADD COLUMN \"{}\" is NOT NULL without DEFAULT and fails on tables that already have rows",
                        column_def.name.value
                    ),
                    help: String::from(
                        "add the column as nullable, backfill it, then add the constraint, or use a constant DEFAULT",
                    ),
                });
            }
        }

        findings
    }
}
