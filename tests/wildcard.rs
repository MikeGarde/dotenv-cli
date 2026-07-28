use assert_cmd::Command;
use std::fs;
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;

fn bin() -> Command {
    Command::cargo_bin("dotenv").unwrap()
}

fn env_path() -> String {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    here.join("tests/.env.test").to_string_lossy().to_string()
}

fn keys_for(pattern: &str) -> Vec<String> {
    let output = bin()
        .arg(pattern)
        .arg("--file")
        .arg(env_path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let value: serde_json::Value = serde_json::from_slice(&output).unwrap();
    value.as_object().unwrap().keys().cloned().collect()
}

#[test]
fn trailing_wildcard_matches_by_prefix() {
    assert_eq!(keys_for("NESTED_*"), vec!["NESTED_VAR1", "NESTED_VAR2"]);
}

#[test]
fn leading_wildcard_matches_by_suffix() {
    assert_eq!(
        keys_for("*_MULTI"),
        vec!["DOUBLE_MULTI", "SINGLE_MULTI", "CORRECT_MULTI"]
    );
}

#[test]
fn wildcard_on_both_sides_matches_the_middle() {
    assert_eq!(keys_for("*_MULTI_*"), vec!["LIST_MULTI_LINE"]);
}

#[test]
fn wildcard_output_resolves_nested_variables() {
    bin()
        .arg("NESTED_VAR2*")
        .arg("--file")
        .arg(env_path())
        .assert()
        .success()
        .stdout("{\"NESTED_VAR2\":\"Hello World\"}\n");
}

#[test]
fn wildcard_forces_json_even_for_a_single_match() {
    bin()
        .arg("NAM*")
        .arg("--file")
        .arg(env_path())
        .assert()
        .success()
        .stdout("{\"NAME\":\"dotenv-cli\"}\n");
}

#[test]
fn wildcard_matching_nothing_returns_an_empty_object_not_the_whole_file() {
    bin()
        .arg("ZZZ_*")
        .arg("--file")
        .arg(env_path())
        .assert()
        .success()
        .stdout("{}\n");
}

#[test]
fn no_json_disables_wildcard_expansion() {
    // With --no-json the pattern is treated as a literal key name, which does
    // not exist, so the lookup fails.
    bin()
        .arg("NESTED_*")
        .arg("--no-json")
        .arg("--file")
        .arg(env_path())
        .assert()
        .failure();
}

const WRITABLE: &[u8] = b"# header\nDB_HOST=localhost\nAPP=1\nDB_USER=root\n";

fn writable_env() -> NamedTempFile {
    let mut tmp = NamedTempFile::new().unwrap();
    tmp.write_all(WRITABLE).unwrap();
    tmp.flush().unwrap();
    tmp
}

/// A wildcard names zero or more keys, so there is no single key to write to or
/// remove. Rather than guessing at which match was meant (or panicking when
/// there is none), the pattern is refused and the file is left untouched.
fn assert_write_rejected(pattern: &str, action: &[&str]) {
    let tmp = writable_env();
    bin()
        .arg(pattern)
        .args(action)
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .code(1)
        .stderr("Cannot use a wildcard key with --set or --delete\n");

    assert_eq!(
        fs::read(tmp.path()).unwrap(),
        WRITABLE,
        "{} {:?} must not modify the file",
        pattern,
        action
    );
}

#[test]
fn wildcard_set_is_rejected() {
    assert_write_rejected("DB_*", &["--set", "zzz"]);
}

#[test]
fn wildcard_set_matching_nothing_is_rejected() {
    assert_write_rejected("ZZZ_*", &["--set", "zzz"]);
}

#[test]
fn wildcard_delete_is_rejected() {
    assert_write_rejected("DB_*", &["--delete"]);
}

#[test]
fn wildcard_delete_matching_nothing_is_rejected() {
    assert_write_rejected("ZZZ_*", &["--delete"]);
}
