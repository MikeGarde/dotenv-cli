use assert_cmd::Command;
use std::path::Path;

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
