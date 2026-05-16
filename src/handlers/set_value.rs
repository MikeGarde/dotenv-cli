use std::fs;
use std::io::Write;

use crate::qualifying_rules::Options;

/// Write (or append) a key=value pair to the .env file.
///
/// If the key already exists, the lines from lineStart to lineEnd (inclusive)
/// are spliced out and replaced — preserving all comments and other keys.
/// If the key is new, `KEY=value\n` is appended to the end of the file.
pub fn set_value(options: &Options) {
    let key = &options.target_keys[0];
    let set_val = options.set_value.as_deref().unwrap_or("");
    let new_line = format!("{}={}", key, set_val);

    let env_object = options.env_object.as_ref().unwrap();

    if let Some(env_val) = env_object.get(key) {
        // Read file fresh (in case concurrent changes occurred)
        let content = fs::read_to_string(&options.full_env_path).expect("Failed to read .env file");
        let mut lines: Vec<String> = content.split('\n').map(str::to_string).collect();

        let start = env_val.line_start as usize;
        let end = env_val.line_end as usize;
        let new_lines: Vec<String> = new_line.split('\n').map(str::to_string).collect();

        // splice: replace [start, end] with new_lines
        lines.splice(start..=end, new_lines);

        fs::write(&options.full_env_path, lines.join("\n")).expect("Failed to write .env file");
    } else {
        // Append new key to end of file
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(&options.full_env_path)
            .expect("Failed to open .env file for appending");
        writeln!(file, "{}", new_line).expect("Failed to append to .env file");
    }
}
