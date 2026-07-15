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

/// If `blob` looks like a JSON array, re-format it as `["a", "b", ...]`.
/// Returns an error if it starts/ends like an array but isn't valid JSON.
fn reformat_json_array_if_present(blob: String, start: usize) -> Result<String, EnvParseError> {
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
                Ok(format!("[{}]", items.join(", ")))
            }
            Err(_) => Err(EnvParseError {
                line: start + 1,
                message: format!("Invalid list: {}", blob),
            }),
        }
    } else {
        Ok(blob)
    }
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

    reformat_json_array_if_present(blob, start)
}

/// Find the line and byte offset of the quote character that closes a quoted
/// value opened by `quote` at `open_pos` on `lines[start]`.
///
/// A candidate closing quote only counts if everything after it on that line
/// (once trimmed) is empty or an inline `# comment` — this lets
/// `KEY="value" # comment` close cleanly instead of being mistaken for an
/// unterminated value, which used to silently swallow subsequent lines/keys
/// into the value while scanning for a line that *ends* with a quote. Any
/// other trailing junk after a quote character is treated the same as before:
/// not closed yet, keep scanning (so a value that legitimately contains a
/// stray quote character mid-value still resolves the same way it always did).
///
/// Errors instead of scanning to EOF if no closing quote is ever found, so an
/// unterminated value fails loudly rather than absorbing the rest of the file.
fn find_quote_close(
    lines: &[&str],
    start: usize,
    open_pos: usize,
    quote: char,
) -> Result<(usize, usize), EnvParseError> {
    let mut line_idx = start;
    let mut search_from = open_pos + quote.len_utf8();
    loop {
        let line = lines[line_idx];
        if let Some(rel) = line[search_from..].rfind(quote) {
            let close_pos = search_from + rel;
            let after = line[close_pos + quote.len_utf8()..].trim_start();
            if after.is_empty() || after.starts_with('#') {
                return Ok((line_idx, close_pos));
            }
        }
        if line_idx + 1 >= lines.len() {
            return Err(EnvParseError {
                line: start + 1,
                message: format!(
                    "Unterminated quoted value starting on line {}: {}",
                    start + 1,
                    lines[start]
                ),
            });
        }
        line_idx += 1;
        search_from = 0;
    }
}

/// Extract the value of a quoted (single- or double-quoted) KEY spanning
/// lines[start..=end_line], given the byte offsets of the opening quote on
/// lines[start] and the closing quote on lines[end_line] (as returned by
/// `find_quote_close`). Everything from the closing quote onward on the last
/// line — including a trailing `# comment` — is discarded.
fn extract_quoted_value(
    lines: &[&str],
    start: usize,
    end_line: usize,
    open_pos: usize,
    close_pos: usize,
    quote: char,
) -> Result<String, EnvParseError> {
    let content_start = open_pos + quote.len_utf8();
    let blob = if start == end_line {
        lines[start][content_start..close_pos].to_string()
    } else {
        let mut parts: Vec<&str> = vec![&lines[start][content_start..]];
        for line in lines.iter().take(end_line).skip(start + 1) {
            parts.push(line);
        }
        parts.push(&lines[end_line][..close_pos]);
        parts.join("\n")
    };

    reformat_json_array_if_present(blob, start)
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
    // Non-structural problems (bad spacing, duplicate keys) are collected rather
    // than failing fast, so `--validate` can report every offending line at once.
    let mut violations: Vec<(usize, String)> = Vec::new();
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

        let key_segment = &trimmed[..eq_pos];
        let value_segment = &trimmed[eq_pos + 1..];
        let key = key_segment.trim().to_string();
        let value_part = value_segment.trim();

        if key.is_empty() {
            i += 1;
            continue;
        }

        // Whitespace hugging the '=' (`KEY = value` or `KEY= value`) is not part
        // of the key or value and is almost always a mistake.
        if key_segment.ends_with(char::is_whitespace) || value_segment.starts_with(char::is_whitespace)
        {
            violations.push((
                line_start + 1,
                format!("whitespace around '=' is not allowed: {}", raw_lines[line_start]),
            ));
        }

        // A key defined more than once is ambiguous.
        if env_object.get(&key).is_some() {
            violations.push((line_start + 1, format!("duplicate key '{}'", key)));
        }

        if value_part.starts_with('"') || value_part.starts_with('\'') {
            let quote = value_part.chars().next().unwrap();
            let first_line = raw_lines[line_start];
            let raw_eq_pos = first_line.find('=').unwrap_or(0);
            let open_pos = first_line[raw_eq_pos..]
                .find(quote)
                .map(|p| raw_eq_pos + p)
                .unwrap_or(raw_eq_pos);
            let (end, close_pos) = find_quote_close(&raw_lines, line_start, open_pos, quote)?;
            let value =
                extract_quoted_value(&raw_lines, line_start, end, open_pos, close_pos, quote)?;
            let mut env_value = EnvValue::with_lines(value, line_start as i64, end as i64);
            // Single quotes disable ${VAR} expansion (literal, POSIX-style).
            env_value.no_expand = quote == '\'';
            env_object.set(key, env_value);
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

    if !violations.is_empty() {
        let first_line = violations[0].0;
        let message = if violations.len() == 1 {
            violations[0].1.clone()
        } else {
            let list: String = violations
                .iter()
                .map(|(l, m)| format!("\n  - line {}: {}", l, m))
                .collect();
            format!("found {} problems:{}", violations.len(), list)
        };
        return Err(EnvParseError {
            line: first_line,
            message,
        });
    }

    env_object.resolve_nested_variables();
    Ok(env_object)
}
