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

// Regression tests for the list equivalent of the quoted-value bug above: the
// closing-bracket search required the line to *end* with ']', so an inline
// comment after the bracket sent it scanning to EOF, swallowing every
// following key into the list value.

#[test]
fn single_line_list_with_trailing_comment_does_not_swallow_next_key() {
    let tmp = write_env("LIST=[\"a\", \"b\"] # inline comment\nNEXT=after\n");
    let json = env_json(&tmp);
    assert_eq!(json["LIST"], "[\"a\", \"b\"]");
    assert_eq!(json["NEXT"], "after");
    assert_eq!(json.as_object().unwrap().len(), 2);
}

#[test]
fn multiline_list_with_comment_on_closing_line_does_not_swallow_next_key() {
    let tmp = write_env("LIST=[\n\"a\",\n\"b\"\n] # inline comment\nNEXT=after\n");
    let json = env_json(&tmp);
    assert_eq!(json["LIST"], "[\"a\", \"b\"]");
    assert_eq!(json["NEXT"], "after");
    assert_eq!(json.as_object().unwrap().len(), 2);
}

#[test]
fn list_item_containing_hash_is_not_mistaken_for_a_comment() {
    let tmp = write_env("LIST=[\"a # b\", \"c\"]\nNEXT=after\n");
    let json = env_json(&tmp);
    assert_eq!(json["LIST"], "[\"a # b\", \"c\"]");
    assert_eq!(json["NEXT"], "after");
}

#[test]
fn multiline_list_keeps_hash_on_interior_lines() {
    let tmp = write_env("LIST=[\n\"a # b\",\n\"c\"\n] # trailing\nNEXT=after\n");
    let json = env_json(&tmp);
    assert_eq!(json["LIST"], "[\"a # b\", \"c\"]");
    assert_eq!(json["NEXT"], "after");
}

// The closing `]` is found by counting bracket depth, not by position on the
// line. Locating it any other way (the last `]`, or one followed only by a
// comment) misreads a bracket that appears inside the trailing comment or
// inside a string.

#[test]
fn a_bracket_inside_the_trailing_comment_does_not_end_the_list_early() {
    let tmp = write_env("LIST=[\"a\"] # see note [1]\nNEXT=after\n");
    let json = env_json(&tmp);
    assert_eq!(json["LIST"], "[\"a\"]");
    assert_eq!(json["NEXT"], "after");
    assert_eq!(json.as_object().unwrap().len(), 2);
}

#[test]
fn a_bracket_inside_a_list_item_does_not_close_the_list() {
    let tmp = write_env("LIST=[\"a]b\", \"c\"]\nNEXT=after\n");
    let json = env_json(&tmp);
    assert_eq!(json["LIST"], "[\"a]b\", \"c\"]");
    assert_eq!(json["NEXT"], "after");
}

#[test]
fn nested_lists_close_on_the_outermost_bracket() {
    let tmp = write_env("LIST=[[\"a\",\"b\"],[\"c\"]] # tail [2]\nNEXT=after\n");
    let json = env_json(&tmp);
    assert_eq!(json["LIST"], "[[\"a\",\"b\"], [\"c\"]]");
    assert_eq!(json["NEXT"], "after");
}

#[test]
fn multiline_nested_list_with_bracket_in_comment_closes_correctly() {
    let tmp = write_env("LIST=[\n\"x]y\",\n[\"n\"]\n] # tail [3]\nNEXT=after\n");
    let json = env_json(&tmp);
    assert_eq!(json["LIST"], "[\"x]y\", [\"n\"]]");
    assert_eq!(json["NEXT"], "after");
}

#[test]
fn unterminated_list_fails_loudly_instead_of_swallowing_rest_of_file() {
    let tmp = write_env("LIST=[\"a\",\nNEXT=after\nMORE=x\n");
    bin()
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Unterminated list starting on line 1",
        ));
}

// `export KEY=value` is the standard form for .env files that double as shell
// sourceable scripts. The prefix used to become part of the key name, so the
// key was unreachable by its real name and `--` injected a variable literally
// named `export FOO`.

#[test]
fn export_prefix_is_not_part_of_the_key() {
    let tmp = write_env("export FOO=bar\nPLAIN=baz\n");
    let json = env_json(&tmp);
    assert_eq!(json["FOO"], "bar");
    assert_eq!(json["PLAIN"], "baz");
    assert!(json.get("export FOO").is_none());
}

#[test]
fn export_prefix_works_for_quoted_and_list_values() {
    let tmp = write_env("export Q=\"quoted\"\nexport L=[\"a\"]\nexport M=\"one\ntwo\"\n");
    let json = env_json(&tmp);
    assert_eq!(json["Q"], "quoted");
    assert_eq!(json["L"], "[\"a\"]");
    assert_eq!(json["M"], "one\ntwo");
}

#[test]
fn export_prefix_requires_whitespace_so_a_key_named_export_still_works() {
    let tmp = write_env("export=value\n");
    let json = env_json(&tmp);
    assert_eq!(json["export"], "value");
}

#[test]
fn exported_key_defined_twice_is_still_a_duplicate() {
    let tmp = write_env("export FOO=bar\nFOO=baz\n");
    bin()
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("duplicate key 'FOO'"));
}

// A key never contains a space, and `export` is the only word allowed to sit in
// front of one. Anything else is rejected rather than silently parsed into a key
// name that no shell could address.

#[test]
fn a_key_containing_a_space_is_rejected() {
    let tmp = write_env("FOO BAR=baz\n");
    bin()
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "invalid key 'FOO BAR': keys cannot contain whitespace",
        ));
}

#[test]
fn export_is_the_only_word_allowed_before_a_key() {
    let tmp = write_env("exprot FOO=baz\n");
    bin()
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "invalid key 'exprot FOO': keys cannot contain whitespace",
        ));
}

#[test]
fn a_second_word_after_export_is_still_rejected() {
    let tmp = write_env("export FOO BAR=baz\n");
    bin()
        .arg("--file")
        .arg(tmp.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "invalid key 'FOO BAR': keys cannot contain whitespace",
        ));
}

#[test]
fn a_key_adjacent_to_export_is_left_alone() {
    let tmp = write_env("exportFOO=a\nexport_ISH=b\nexport   SPACED=c\n");
    let json = env_json(&tmp);
    assert_eq!(json["exportFOO"], "a");
    assert_eq!(json["export_ISH"], "b");
    assert_eq!(json["SPACED"], "c");
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
