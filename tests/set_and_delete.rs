use assert_cmd::Command;
use std::fs;
use std::path::Path;
use tempfile::NamedTempFile;

fn bin() -> Command {
    Command::cargo_bin("dotenv").unwrap()
}

fn env_path() -> String {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    here.join("tests/.env.test").to_string_lossy().to_string()
}

fn copy_env() -> NamedTempFile {
    let orig = env_path();
    let mut tmp = NamedTempFile::new().unwrap();
    let content = fs::read_to_string(&orig).unwrap();
    use std::io::Write;
    tmp.write_all(content.as_bytes()).unwrap();
    tmp
}

fn env_json(env_file: &NamedTempFile) -> serde_json::Value {
    let output = bin().arg("--file").arg(env_file.path()).output().unwrap();
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn add_a_key() {
    let tmp = copy_env();
    bin()
        .arg("NEW_KEY")
        .arg("--set")
        .arg("VERY_NEW")
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .success();
    bin()
        .arg("NEW_KEY")
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout("VERY_NEW\n");
}

#[test]
fn delete_existing_key() {
    let tmp = copy_env();
    bin()
        .arg("NEW_KEY")
        .arg("--set")
        .arg("VERY_NEW")
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .success();
    bin()
        .arg("NEW_KEY")
        .arg("--delete")
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .success();
    let json = env_json(&tmp);
    assert!(
        !json.as_object().unwrap().contains_key("NEW_KEY"),
        "Key 'NEW_KEY' should have been deleted, but is still present in the env file"
    );
}

#[test]
fn add_multiline_value_single_line() {
    let tmp = copy_env();
    bin()
        .arg("NEW_ONE")
        .arg("--set")
        .arg("This is a\\nmultiline value")
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .success();
    bin()
        .arg("NEW_ONE")
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout("This is a\\nmultiline value\n");
}

#[test]
fn update_existing_key() {
    let tmp = copy_env();
    bin()
        .arg("NEW_ONE")
        .arg("--set")
        .arg("Single line value")
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .success();
    let json = env_json(&tmp);
    assert_eq!(
        json["NEW_ONE"], "Single line value",
        "Key 'NEW_ONE' should have value 'Single line value', got {:?}",
        json["NEW_ONE"]
    );
}

#[test]
fn update_existing_key_with_stdin() {
    let tmp = copy_env();
    // `--set -` opts into reading the value from stdin
    bin()
        .arg("NEW_TWO")
        .arg("--set")
        .arg("-")
        .arg("--file")
        .arg(tmp.path())
        .write_stdin("New stdin value")
        .assert()
        .success();
    bin()
        .arg("NEW_TWO")
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout("New stdin value\n");
}

#[test]
fn read_ignores_piped_stdin() {
    // A plain read must never consume stdin, even when something is piped in.
    let tmp = copy_env();
    bin()
        .arg("NEW_ONE")
        .arg("--set")
        .arg("Single line value")
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .success();
    bin()
        .arg("NEW_ONE")
        .arg("--file")
        .arg(tmp.path())
        .write_stdin("this should be ignored")
        .assert()
        .success()
        .stdout("Single line value\n");
}

#[test]
fn add_list() {
    let tmp = copy_env();
    bin()
        .arg("LIST")
        .arg("--set")
        .arg("[\"one\", \"two\", \"three\"]")
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .success();
    bin()
        .arg("LIST")
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout("[\"one\", \"two\", \"three\"]\n");
}

#[test]
fn update_list() {
    let tmp = copy_env();
    bin()
        .arg("LIST")
        .arg("--set")
        .arg("[\"four\", \"five\", \"six\"]")
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .success();
    bin()
        .arg("LIST")
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout("[\"four\", \"five\", \"six\"]\n");
}

#[test]
fn remove_all_new_test_keys() {
    let tmp = copy_env();
    for (key, value) in [
        ("NEW_ONE", "one"),
        ("NEW_TWO", "two"),
        ("LIST", "[\"one\", \"two\", \"three\"]"),
    ] {
        bin()
            .arg(key)
            .arg("--set")
            .arg(value)
            .arg("--file")
            .arg(tmp.path())
            .assert()
            .success();
    }

    for key in &["NEW_ONE", "NEW_TWO", "LIST"] {
        bin()
            .arg(key)
            .arg("--delete")
            .arg("--file")
            .arg(tmp.path())
            .assert()
            .success();
    }
    let json = env_json(&tmp);
    let keys = json.as_object().unwrap();
    assert!(
        !keys.contains_key("NEW_ONE"),
        "Key 'NEW_ONE' should have been deleted, but is still present"
    );
    assert!(
        !keys.contains_key("NEW_TWO"),
        "Key 'NEW_TWO' should have been deleted, but is still present"
    );
    assert!(
        !keys.contains_key("LIST"),
        "Key 'LIST' should have been deleted, but is still present"
    );
}

#[test]
fn chained_set_update_delete_workflow_preserves_file() {
    use sha2::{Digest, Sha256};
    let orig = fs::read(env_path()).unwrap();
    let tmp = copy_env();
    let hash1 = Sha256::digest(&orig);

    bin()
        .arg("NEW_KEY")
        .arg("--set")
        .arg("VERY_NEW")
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .success();
    bin()
        .arg("NEW_KEY")
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout("VERY_NEW\n");
    bin()
        .arg("NEW_KEY")
        .arg("--delete")
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .success();
    assert!(
        !env_json(&tmp).as_object().unwrap().contains_key("NEW_KEY"),
        "NEW_KEY should be absent after deletion"
    );

    bin()
        .arg("NEW_ONE")
        .arg("--set")
        .arg("This is a\\nmultiline value")
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .success();
    bin()
        .arg("NEW_ONE")
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout("This is a\\nmultiline value\n");

    bin()
        .arg("NEW_TWO")
        .arg("--set")
        .arg("This is a\\nmultiline value")
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .success();

    bin()
        .arg("NEW_ONE")
        .arg("--set")
        .arg("Single line value")
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .success();
    let json = env_json(&tmp);
    let keys: Vec<_> = json.as_object().unwrap().keys().cloned().collect();
    let last_key = keys.last().unwrap();
    assert_eq!(json["NEW_ONE"], "Single line value");
    assert_eq!(last_key, "NEW_TWO");
    assert_eq!(json["NEW_TWO"], "This is a\\nmultiline value");

    bin()
        .arg("NEW_TWO")
        .arg("--set")
        .arg("-")
        .arg("--file")
        .arg(tmp.path())
        .write_stdin("New stdin value")
        .assert()
        .success();
    bin()
        .arg("NEW_TWO")
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout("New stdin value\n");

    bin()
        .arg("LIST")
        .arg("--set")
        .arg("[\"one\", \"two\", \"three\"]")
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .success();
    bin()
        .arg("LIST")
        .arg("--set")
        .arg("[\"four\", \"five\", \"six\"]")
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .success();
    bin()
        .arg("LIST")
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .success()
        .stdout("[\"four\", \"five\", \"six\"]\n");

    for key in &["NEW_ONE", "NEW_TWO", "LIST"] {
        bin()
            .arg(key)
            .arg("--delete")
            .arg("--file")
            .arg(tmp.path())
            .assert()
            .success();
    }

    let json = env_json(&tmp);
    let keys = json.as_object().unwrap();
    assert!(!keys.contains_key("NEW_ONE"));
    assert!(!keys.contains_key("NEW_TWO"));
    assert!(!keys.contains_key("LIST"));

    let new = fs::read(tmp.path()).unwrap();
    let hash2 = Sha256::digest(&new);
    assert_eq!(
        hash1[..],
        hash2[..],
        ".env file changed after the full set/update/delete workflow: original hash = {:x?}, new hash = {:x?}",
        hash1,
        hash2
    );
}

#[test]
fn add_and_delete_single_key_preserves_file() {
    use sha2::{Digest, Sha256};
    let orig = fs::read(env_path()).unwrap();
    let tmp = copy_env();
    let hash1 = Sha256::digest(&orig);

    bin()
        .arg("HASH_TEST")
        .arg("--set")
        .arg("value")
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .success();
    bin()
        .arg("HASH_TEST")
        .arg("--delete")
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .success();

    let new = fs::read(tmp.path()).unwrap();
    let hash2 = Sha256::digest(&new);
    assert_eq!(
        hash1[..],
        hash2[..],
        ".env file changed after add+delete: original hash = {:x?}, new hash = {:x?}",
        hash1,
        hash2
    );
}
