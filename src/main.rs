mod parser;
mod rule;
mod rules;

use anyhow::{Context, Result};
use clap::Parser;
use std::{path::PathBuf, process::ExitCode};

use crate::rule::{Finding, Rule, Severity};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    path: PathBuf,
}

fn lint(content: &str, rules: &[Box<dyn Rule>]) -> Vec<Finding> {
    let mut findings = Vec::new();
    for stmt in parser::split_statements(content) {
        for rule in rules {
            findings.extend(rule.check(&stmt));
        }
    }
    findings
}

fn main() -> Result<ExitCode> {
    let cli = Cli::parse();

    let content = std::fs::read_to_string(&cli.path)
        .with_context(|| format!("failed not read {}", cli.path.display()))?;

    let findings = lint(&content, &rules::all());

    for finding in &findings {
        println!("{}:{}", cli.path.display(), finding);
    }

    let errors = findings
        .iter()
        .filter(|f| f.severity == Severity::Error)
        .count();
    println!("{} warning(s), {} error(s)", findings.len(), errors);

    Ok(if errors > 0 {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}
