use sqlparser::ast::Statement;
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::{Parser, ParserError};

#[derive(Debug)]
pub struct ParsedStatement {
    pub ast: Statement,
    pub line: usize,
}

pub fn parse(sql: &str) -> Result<Vec<ParsedStatement>, ParserError> {
    let statements = Parser::parse_sql(&PostgreSqlDialect {}, sql)?;

    Ok(statements
        .into_iter()
        .map(|ast| ParsedStatement { ast, line: 0 })
        .collect())
}