use assert_cmd::Command;
use predicates::prelude::*;
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

fn write_env(content: &str) -> NamedTempFile {
    let mut tmp = NamedTempFile::new().unwrap();
    tmp.write_all(content.as_bytes()).unwrap();
    tmp
}

fn env_json(tmp: &NamedTempFile) -> serde_json::Value {
    let output = bin().arg("--file").arg(tmp.path()).output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str(stdout.trim())
        .unwrap_or_else(|e| panic!("expected valid JSON, got {:?}: {}", stdout, e))
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

// Regression tests for a bug where a trailing inline `# comment` after a
// quoted value's closing quote defeated the "does the line end with a quote"
// check used to find that value's end. The parser would then keep scanning
// forward for a line that literally ended in a quote character, silently
// swallowing the next key's line into the value (and truncating it besides).

#[test]
fn double_quoted_value_with_trailing_comment_does_not_swallow_next_key() {
    let tmp = write_env("DOUBLE=\"Double quotes\" # inline comment\nNEXT=after\n");
    let json = env_json(&tmp);
    assert_eq!(json["DOUBLE"], "Double quotes");
    assert_eq!(json["NEXT"], "after");
    assert_eq!(json.as_object().unwrap().len(), 2);
}

#[test]
fn single_quoted_value_with_trailing_comment_does_not_swallow_next_key() {
    let tmp = write_env("SINGLE='Single quotes' # inline comment\nNEXT=after\n");
    let json = env_json(&tmp);
    assert_eq!(json["SINGLE"], "Single quotes");
    assert_eq!(json["NEXT"], "after");
    assert_eq!(json.as_object().unwrap().len(), 2);
}

#[test]
fn multiline_quoted_value_with_trailing_comment_on_closing_line() {
    let tmp = write_env("MULTI=\"line one\nline two\" # inline comment\nNEXT=after\n");
    let json = env_json(&tmp);
    assert_eq!(json["MULTI"], "line one\nline two");
    assert_eq!(json["NEXT"], "after");
    assert_eq!(json.as_object().unwrap().len(), 2);
}

#[test]
fn quoted_value_with_no_trailing_comment_is_unaffected() {
    // Guards against the fix changing behaviour for the common (no-comment) case.
    let tmp = write_env("DOUBLE=\"Double quotes\"\nNEXT=after\n");
    let json = env_json(&tmp);
    assert_eq!(json["DOUBLE"], "Double quotes");
    assert_eq!(json["NEXT"], "after");
}

#[test]
fn unquoted_value_keeps_trailing_hash_literally() {
    // dotenv-cli has never supported inline comments after unquoted values;
    // unlike the quoted case, an unquoted value is always a single line, so
    // there's no risk of it swallowing a following key. Everything after '='
    // is taken as the literal value, `#` included.
    let tmp = write_env("PLAIN=bar # not a comment\nNEXT=after\n");
    let json = env_json(&tmp);
    assert_eq!(json["PLAIN"], "bar # not a comment");
    assert_eq!(json["NEXT"], "after");
}

#[test]
fn unterminated_quoted_value_fails_loudly_instead_of_swallowing_rest_of_file() {
    let tmp = write_env("BROKEN=\"never closed\nNEXT=after\n");
    bin()
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Unterminated quoted value starting on line 1",
        ));
}

#[test]
fn double_quotes_expand_variables() {
    let tmp = write_env("BASE=Hello\nEXPAND=\"${BASE} World\"\n");
    let json = env_json(&tmp);
    assert_eq!(json["EXPAND"], "Hello World");
}

#[test]
fn single_quotes_disable_variable_expansion() {
    let tmp = write_env("BASE=Hello\nLITERAL='${BASE} World'\n");
    let json = env_json(&tmp);
    assert_eq!(json["LITERAL"], "${BASE} World");
}

#[test]
fn single_quotes_preserve_password_containing_dollar() {
    // A password that looks like a variable reference must be stored verbatim.
    let tmp = write_env("BASE=Hello\nPASSWORD='p${BASE}ss$w0rd$'\n");
    let json = env_json(&tmp);
    assert_eq!(json["PASSWORD"], "p${BASE}ss$w0rd$");
}

#[test]
fn multiline_single_quotes_disable_expansion() {
    let tmp = write_env("BASE=Hello\nLITERAL='${BASE}\nsecond line'\n");
    let json = env_json(&tmp);
    assert_eq!(json["LITERAL"], "${BASE}\nsecond line");
}

fn single_quote_fixture() -> String {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    here.join("tests/envFiles/singleQuote.env")
        .to_string_lossy()
        .to_string()
}

#[test]
fn single_quote_fixture_expansion_rules() {
    let output = bin()
        .arg("--file")
        .arg(single_quote_fixture())
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap();
    assert_eq!(json["EXPAND_DOUBLE"], "Hello World");
    assert_eq!(json["LITERAL_SINGLE"], "${BASE} World");
    assert_eq!(json["PASSWORD"], "p${BASE}ss$w0rd$");
    assert_eq!(json["BARE_DOLLAR"], "pa$$w0rd");
}
