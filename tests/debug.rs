use assert_cmd::Command;
use predicates::prelude::*;
use std::path::Path;

fn bin() -> Command {
    Command::cargo_bin("dotenv").unwrap()
}

fn env_path() -> String {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    here.join("tests/.env.test").to_string_lossy().to_string()
}

#[test]
fn debug_reports_keys_and_file() {
    bin()
        .arg("NAME")
        .arg("--debug")
        .arg("--file")
        .arg(env_path())
        .assert()
        .success()
        .stdout("dotenv-cli\n")
        .stderr(
            predicate::str::contains("Keys: [\"NAME\"]")
                .and(predicate::str::contains(".env.test"))
                .and(predicate::str::contains("Options assembled")),
        );
}

#[test]
fn debug_reports_json_defaulting_and_wildcards() {
    bin()
        .arg("NESTED_*")
        .arg("--debug")
        .arg("--file")
        .arg(env_path())
        .assert()
        .success()
        .stderr(predicate::str::contains("Wildcard found"));
}

#[test]
fn debug_reports_json_defaulting_for_multiple_keys() {
    bin()
        .arg("NAME")
        .arg("EMPTY")
        .arg("--debug")
        .arg("--file")
        .arg(env_path())
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "Key count (0 or >1) defaulting to JSON",
        ));
}

#[test]
fn debug_reports_file_during_validate() {
    bin()
        .arg("--validate")
        .arg("--debug")
        .arg("--file")
        .arg(env_path())
        .assert()
        .success()
        .stderr(predicate::str::contains("File: "));
}

#[test]
fn debug_reports_command_before_running_it() {
    bin()
        .arg("--debug")
        .arg("--file")
        .arg(env_path())
        .arg("--")
        .arg("true")
        .assert()
        .success()
        .stderr(
            predicate::str::contains("Command: [\"true\"]")
                .and(predicate::str::contains("Running: true")),
        );
}

#[test]
fn debug_reports_variables_skipped_because_already_set() {
    bin()
        .arg("--debug")
        .arg("--file")
        .arg(env_path())
        .env("NAME", "from-the-shell")
        .arg("--")
        .arg("true")
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "Skipping NAME (already set in the environment)",
        ));
}
