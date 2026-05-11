use assert_cmd::Command;
use std::path::Path;

fn bin() -> Command {
    Command::cargo_bin("dotenv").unwrap()
}

fn env_path() -> String {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    here.join("tests/.env.test").to_string_lossy().to_string()
}

#[test]
fn output_entire_env_as_json() {
    let output = bin()
        .arg("--json")
        .arg("--file")
        .arg(env_path())
        .output()
        .expect("failed to run dotenv");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
    let length = json.as_object().unwrap().len();
    assert_eq!(json["NAME"], "dotenv-cli");
    assert!(length > 1);
}

#[test]
fn output_single_key_as_json() {
    let output = bin()
        .arg("NAME")
        .arg("--json")
        .arg("--file")
        .arg(env_path())
        .output()
        .expect("failed to run dotenv");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
    let length = json.as_object().unwrap().len();
    assert_eq!(json["NAME"], "dotenv-cli");
    assert_eq!(length, 1);
}

#[test]
fn multiple_keys_as_json() {
    let output = bin()
        .arg("NAME")
        .arg("DOUBLE")
        .arg("--file")
        .arg(env_path())
        .output()
        .expect("failed to run dotenv");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
    let length = json.as_object().unwrap().len();
    assert_eq!(json["NAME"], "dotenv-cli");
    assert_eq!(json["DOUBLE"], "Double quotes");
    assert_eq!(length, 2);
}
