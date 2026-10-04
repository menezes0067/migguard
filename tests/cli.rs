use std::process::{Command, Output};

fn run(fixture: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_migguard"))
        .arg(format!("tests/fixtures/{fixture}"))
        .output()
        .expect("falha ao executar o binário")
}

#[test]
fn good_migration_passes() {
    let out = run("good.sql");
    assert!(out.status.success());
}

#[test]
fn bad_migration_fails_and_reports_rule() {
    let out = run("bad.sql");
    let stdout = String::from_utf8_lossy(&out.stdout);

    assert!(!out.status.success());
    assert!(stdout.contains("create_index_concurrently"));
    assert!(stdout.contains("bad.sql"));
}
