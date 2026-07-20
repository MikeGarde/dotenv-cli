use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

fn bin() -> Command {
    Command::cargo_bin("dotenv").unwrap()
}

fn env_file(content: &str) -> NamedTempFile {
    let mut tmp = NamedTempFile::new().unwrap();
    tmp.write_all(content.as_bytes()).unwrap();
    tmp
}

#[test]
fn injects_variables_into_command() {
    let env = env_file("GREETING=hello\nNAME=world\n");
    bin()
        .arg("--file")
        .arg(env.path())
        .arg("--")
        .arg("sh")
        .arg("-c")
        .arg("printf '%s %s' \"$GREETING\" \"$NAME\"")
        .assert()
        .success()
        .stdout("hello world");
}

#[test]
fn resolves_nested_variables() {
    let env = env_file("GREETING=hello\nNAME=world\nNESTED=${GREETING}-${NAME}\n");
    bin()
        .arg("--file")
        .arg(env.path())
        .arg("--")
        .arg("sh")
        .arg("-c")
        .arg("printf '%s' \"$NESTED\"")
        .assert()
        .success()
        .stdout("hello-world");
}

#[test]
fn passes_through_exit_code() {
    let env = env_file("FOO=bar\n");
    bin()
        .arg("--file")
        .arg(env.path())
        .arg("--")
        .arg("sh")
        .arg("-c")
        .arg("exit 3")
        .assert()
        .code(3);
}

#[cfg(unix)]
#[test]
fn signal_terminated_command_exits_128_plus_signal() {
    let env = env_file("FOO=bar\n");
    // SIGTERM is 15, so shells report 143.
    bin()
        .arg("--file")
        .arg(env.path())
        .arg("--")
        .arg("bash")
        .arg("-c")
        .arg("kill -TERM $$; sleep 5")
        .assert()
        .code(143);
}

#[test]
fn existing_environment_takes_precedence() {
    let env = env_file("GREETING=fromfile\n");
    bin()
        .arg("--file")
        .arg(env.path())
        .env("GREETING", "fromshell")
        .arg("--")
        .arg("sh")
        .arg("-c")
        .arg("printf '%s' \"$GREETING\"")
        .assert()
        .success()
        .stdout("fromshell");
}

#[test]
fn missing_command_binary_exits_127() {
    let env = env_file("FOO=bar\n");
    bin()
        .arg("--file")
        .arg(env.path())
        .arg("--")
        .arg("this_command_does_not_exist_xyz")
        .assert()
        .code(127)
        .stderr(predicate::str::contains("failed to run"));
}

#[test]
fn missing_env_file_reports_error() {
    bin()
        .arg("--file")
        .arg("non-existent.env")
        .arg("--")
        .arg("echo")
        .arg("hi")
        .assert()
        .failure()
        .stderr(predicate::str::contains("File not found"));
}

#[test]
fn rejects_command_combined_with_key() {
    let env = env_file("FOO=bar\n");
    bin()
        .arg("--file")
        .arg(env.path())
        .arg("KEY")
        .arg("--")
        .arg("echo")
        .arg("hi")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Cannot combine a command"));
}
