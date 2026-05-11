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

fn bad_list_path() -> String {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    here.join("tests/envFiles/badList.env")
        .to_string_lossy()
        .to_string()
}

#[test]
fn missing_env_file() {
    bin()
        .arg("void")
        .arg("--file")
        .arg("non-existent.env")
        .assert()
        .failure()
        .stderr(predicate::str::contains("File not found"));
}

#[test]
fn uses_dotenv_file_env_var() {
    let env_path = env_path();
    bin()
        .arg("NAME")
        .env("DOTENV_FILE", &env_path)
        .assert()
        .success()
        .stdout("dotenv-cli\n");
}

#[test]
fn read_simple_value() {
    bin()
        .arg("NAME")
        .arg("--file")
        .arg(env_path())
        .assert()
        .success()
        .stdout("dotenv-cli\n");
}

#[test]
fn read_double_quoted_value() {
    bin()
        .arg("DOUBLE")
        .arg("--file")
        .arg(env_path())
        .assert()
        .success()
        .stdout("Double quotes\n");
}

#[test]
fn read_single_quoted_value() {
    bin()
        .arg("SINGLE")
        .arg("--file")
        .arg(env_path())
        .assert()
        .success()
        .stdout("Single quotes\n");
}

#[test]
fn missing_key() {
    bin()
        .arg("MISSING")
        .arg("--file")
        .arg(env_path())
        .assert()
        .failure()
        .stdout("\n");
}

#[test]
fn valid_single_line_list() {
    bin()
        .arg("LIST_SINGLE_LINE")
        .arg("--file")
        .arg(env_path())
        .assert()
        .success()
        .stdout("[\"one\", \"two\", \"three\"]\n");
}

#[test]
fn valid_multi_line_list() {
    bin()
        .arg("LIST_MULTI_LINE")
        .arg("--file")
        .arg(env_path())
        .assert()
        .success()
        .stdout("[\"one\", \"two\", \"three\"]\n");
}

#[test]
fn invalid_list_throws_error() {
    bin()
        .arg("BAD_LIST")
        .arg("--file")
        .arg(bad_list_path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("EnvParseError"));
}
