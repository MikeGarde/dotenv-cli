use std::fs;

use crate::env_object::{EnvObject, EnvValue};
use crate::errors::EnvParseError;

/// Walk forward from `start` until a line whose trimmed form ends with `end_str`.
/// Returns the index of that line, capped at the last line index.
fn get_end_line(lines: &[&str], start: usize, end_str: &str) -> usize {
    let mut end = start;
    while end < lines.len() && !lines[end].trim().ends_with(end_str) {
        if end + 1 >= lines.len() {
            break;
        }
        end += 1;
    }
    end
}

/// Extract and normalise the value spanning lines[start..=end].
///
/// - Takes everything after the first '=' on lines[start] (preserving '=' in values).
/// - Joins continuation lines with '\n'.
/// - If `quoted`, strips one layer of outer quote characters.
/// - If the result looks like a JSON array, re-formats it as `["a", "b", ...]`
///   and returns an error if the JSON is invalid.
fn extract_value(
    lines: &[&str],
    start: usize,
    end: usize,
    quoted: bool,
) -> Result<String, EnvParseError> {
    let first_line = lines[start];
    let after_eq = first_line
        .find('=')
        .map(|i| &first_line[i + 1..])
        .unwrap_or("");

    let mut parts: Vec<&str> = vec![after_eq];
    for line in lines.iter().take(end + 1).skip(start + 1) {
        parts.push(line);
    }

    let mut blob = parts.join("\n").trim().to_string();

    if quoted && blob.len() >= 2 {
        // Strip outer quote character (first and last byte – safe for ASCII " and ')
        blob = blob[1..blob.len() - 1].to_string();
    }

    // Handle JSON array values: reformat as ["a", "b", ...]
    if blob.starts_with('[') && blob.ends_with(']') {
        match serde_json::from_str::<Vec<serde_json::Value>>(&blob) {
            Ok(arr) => {
                let items: Vec<String> = arr
                    .iter()
                    .map(|item| match item {
                        serde_json::Value::String(s) => format!("\"{}\"", s),
                        other => other.to_string(),
                    })
                    .collect();
                blob = format!("[{}]", items.join(", "));
            }
            Err(_) => {
                return Err(EnvParseError {
                    line: start + 1,
                    message: format!("Invalid list: {}", blob),
                });
            }
        }
    }

    Ok(blob)
}

/// Parse a .env file and return an EnvObject with resolved nested variables.
///
/// Handles:
/// - `KEY=unquoted`
/// - `KEY="double quoted"` (single or multiline)
/// - `KEY='single quoted'` (single or multiline)
/// - `KEY=["json", "array"]` (single or multiline)
/// - Comments (`# …`) and blank lines are ignored.
/// - Lines without `=` are ignored (e.g. `// invalid comment`).
pub fn parse_env_file(file_path: &str) -> Result<EnvObject, EnvParseError> {
    let content = fs::read_to_string(file_path).map_err(|e| EnvParseError {
        line: 0,
        message: e.to_string(),
    })?;

    // Keep raw lines for line-number-based splice operations.
    let raw_lines: Vec<&str> = content.split('\n').collect();
    let mut env_object = EnvObject::new();
    let mut i = 0;

    while i < raw_lines.len() {
        let trimmed = raw_lines[i].trim();
        let line_start = i;

        // Skip blank lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') {
            i += 1;
            continue;
        }

        // Skip lines without '=' (e.g. `// invalid comment`)
        let eq_pos = match trimmed.find('=') {
            Some(p) => p,
            None => {
                i += 1;
                continue;
            }
        };

        let key = trimmed[..eq_pos].to_string();
        let value_part = trimmed[eq_pos + 1..].trim();

        if key.is_empty() {
            i += 1;
            continue;
        }

        if value_part.starts_with('"') {
            let end = get_end_line(&raw_lines, line_start, "\"");
            let value = extract_value(&raw_lines, line_start, end, true)?;
            env_object.set(
                key,
                EnvValue::with_lines(value, line_start as i64, end as i64),
            );
            i = end + 1;
        } else if value_part.starts_with('\'') {
            let end = get_end_line(&raw_lines, line_start, "'");
            let value = extract_value(&raw_lines, line_start, end, true)?;
            env_object.set(
                key,
                EnvValue::with_lines(value, line_start as i64, end as i64),
            );
            i = end + 1;
        } else if value_part.starts_with('[') {
            let end = get_end_line(&raw_lines, line_start, "]");
            let value = extract_value(&raw_lines, line_start, end, false)?;
            env_object.set(
                key,
                EnvValue::with_lines(value, line_start as i64, end as i64),
            );
            i = end + 1;
        } else {
            // Unquoted value — must not contain bare quotes (would indicate a parse issue)
            if value_part.contains('"') || value_part.contains('\'') {
                return Err(EnvParseError {
                    line: i + 1,
                    message: format!("Invalid value: {}", raw_lines[i]),
                });
            }
            let value = extract_value(&raw_lines, line_start, line_start, false)?;
            env_object.set(
                key,
                EnvValue::with_lines(value, line_start as i64, line_start as i64),
            );
            i += 1;
        }
    }

    env_object.resolve_nested_variables();
    Ok(env_object)
}
