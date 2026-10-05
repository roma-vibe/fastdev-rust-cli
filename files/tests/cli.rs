//! Black-box tests: run the compiled binary like a user does and check stdout, stderr and the exit
//! code. Logic is unit-tested next to the code in `src/`.

use std::fs;

use assert_cmd::Command;
use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use serde_json::{Value, json};
use tempfile::TempDir;

/// The binary under test, unaffected by the caller's `RUST_LOG`.
fn tool() -> Command {
    let mut command = cargo_bin_cmd!();
    command.env_remove("RUST_LOG");
    command
}

/// A temporary folder with the given files; it is deleted when the value is dropped.
fn folder_with(files: &[(&str, &str)]) -> TempDir {
    let dir = TempDir::new().expect("create a temporary folder");
    for (name, content) in files {
        fs::write(dir.path().join(name), content).expect("write a test file");
    }
    dir
}

#[test]
fn help_shows_the_app_name_and_the_commands() {
    tool()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("APP_NAME")))
        .stdout(predicate::str::contains("stats"));
}

#[test]
fn version_prints_the_binary_name_and_version() {
    tool().arg("--version").assert().success().stdout(format!(
        "{} {}\n",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    ));
}

#[test]
fn stats_counts_a_file() {
    let dir = folder_with(&[("poem.txt", "roses are red\nviolets are blue\n")]);
    tool()
        .current_dir(dir.path())
        .args(["stats", "poem.txt"])
        .assert()
        .success()
        .stdout("lines words chars bytes  name\n    2     6    31    31  poem.txt\n");
}

#[test]
fn stats_reads_standard_input_without_files() {
    tool()
        .arg("stats")
        .write_stdin("one two three\n")
        .assert()
        .success()
        .stdout(predicate::str::ends_with("    1     3    14    14  -\n"));
}

#[test]
fn stats_prints_json_with_a_total() {
    let dir = folder_with(&[("a.txt", "a b\n"), ("b.txt", "c\n")]);
    let output = tool().current_dir(dir.path()).args(["stats", "--format", "json", "a.txt", "b.txt"]).output().unwrap();
    assert!(output.status.success());

    let result: Value = serde_json::from_slice(&output.stdout).expect("stdout is JSON");
    assert_eq!(result["inputs"][0]["name"], "a.txt");
    assert_eq!(result["inputs"][1]["words"], 1);
    assert_eq!(result["total"], json!({ "lines": 2, "words": 3, "chars": 6, "bytes": 6 }));
}

#[test]
fn stats_fails_with_exit_code_1_on_a_missing_file() {
    let dir = folder_with(&[]);
    tool()
        .current_dir(dir.path())
        .args(["stats", "missing.txt"])
        .assert()
        .code(1)
        .stdout("")
        .stderr(predicate::str::starts_with("error: cannot read missing.txt"));
}

#[test]
fn invalid_arguments_exit_with_code_2() {
    tool().args(["stats", "--format", "xml"]).assert().code(2).stderr(predicate::str::contains("invalid value 'xml'"));
}

#[test]
fn logs_go_to_stderr_and_keep_stdout_clean() {
    tool()
        .args(["stats", "-vv", "--format", "json"])
        .write_stdin("x\n")
        .assert()
        .success()
        .stdout(predicate::str::starts_with("{"))
        .stderr(predicate::str::contains("counted"));
}
