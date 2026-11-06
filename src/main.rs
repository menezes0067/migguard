mod parser;
mod rule;
mod rules;

use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::{Context, Result};
use clap::Parser;
use rule::{Finding, Rule, Severity};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    path: PathBuf,
}

fn lint(content: &str, rules: &[Box<dyn Rule>]) -> Result<Vec<Finding>> {
    let statements = parser::parse(content)?;

    let mut findings = Vec::new();
    for stmt in &statements {
        for rule in rules {
            findings.extend(rule.check(stmt));
        }
    }
    Ok(findings)
}

fn main() -> Result<ExitCode> {
    let cli = Cli::parse();

    let content = std::fs::read_to_string(&cli.path)
        .with_context(|| format!("could not read {}", cli.path.display()))?;

    let findings = lint(&content, &rules::all())
        .with_context(|| format!("could not analyze {}", cli.path.display()))?;

    for finding in &findings {
        println!("{}:{}", cli.path.display(), finding);
    }

    let errors = findings
        .iter()
        .filter(|f| f.severity == Severity::Error)
        .count();
    println!("{} finding(s), {} error(s)", findings.len(), errors);

    Ok(if errors > 0 {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}