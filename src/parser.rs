#[derive(Debug)]
pub struct Statement {
    pub sql: String,
    pub line: usize,
}

pub fn split_statements(input: &str) -> Vec<Statement> {
    let cleaned: String = remove_comment(input);

    let mut statements = Vec::new();

    for piece in cleaned.split(';') {
        let sql = piece.trim();
        if sql.is_empty() {
            continue;
        }
        statements.push(Statement {
            sql: sql.to_string(),
            line: 1,
        });
    }

    statements
}

fn remove_comment(input: &str) -> String {
    let mut cleaner = String::new();
    for line in input.lines() {
        let code = line.split("--").next().unwrap_or("");
        cleaner.push_str(code);
        cleaner.push('\n');
    }

    cleaner
}
