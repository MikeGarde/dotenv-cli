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
fn parse_file_count_keys() {
    // The Node test expects >=9 keys due to race conditions, so we just check >=9
    let output = bin()
        .arg("--file")
        .arg(env_path())
        .output()
        .expect("failed to run dotenv");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
    let count = json.as_object().unwrap().len();
    assert!(count >= 9, "expected at least 9 keys, got {}", count);
}
