use dotenv_cli::format_value::format_value;

#[test]
fn single_line_to_literal() {
    let value = "line1\nline2";
    let formatted = format_value(value, false);
    assert_eq!(formatted, "line1\\nline2");
}

#[test]
fn literal_to_multiline() {
    let value = "line1\\nline2";
    let formatted = format_value(value, true);
    assert_eq!(formatted, "line1\nline2");
}
