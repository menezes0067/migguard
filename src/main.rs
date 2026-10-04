mod parser;
mod rule;
mod rules;

use anyhow::{Context, Result};
use clap::Parser;
use std::{path::PathBuf, process::ExitCode};

use crate::rule::Severity;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    path: PathBuf,
}

fn main() -> Result<ExitCode> {
    let cli = Cli::parse();

    let content = std::fs::read_to_string(&cli.path)
        .with_context(|| format!("não foi possível ler {}", cli.path.display()))?;

    let statements = parser::split_statements(&content);
    let mut errors: i32 = 0;

    for stmt in &statements {
        if let Some(finding) = rules::create_index_concurrently::check(stmt) {
            println!(
                "{}:{}: {} [{}] {}\n    advice: {}\n",
                cli.path.display(),
                finding.line,
                finding.severity,
                finding.rule,
                finding.message,
                finding.help
            );
            if finding.severity == Severity::Error {
                errors += 1;
            }
        }
    }

    println!(
        "{} statement(s) analyzed: {} error(s)",
        statements.len(),
        errors
    );

    Ok(if errors > 0 {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}
