use std::fs;

use crate::qualifying_rules::Options;

/// Remove a key and its value lines from the .env file.
///
/// Returns `true` if the key was found and deleted, `false` if not found
/// (caller should set exit code 1).
pub fn delete_key(options: &Options) -> bool {
    let key = &options.target_keys[0];
    let env_object = options.env_object.as_ref().unwrap();

    if let Some(env_val) = env_object.get(key) {
        let content = fs::read_to_string(&options.full_env_path).expect("Failed to read .env file");
        let mut lines: Vec<String> = content.split('\n').map(str::to_string).collect();

        let start = env_val.line_start as usize;
        let end = env_val.line_end as usize;
        lines.drain(start..=end);

        fs::write(&options.full_env_path, lines.join("\n")).expect("Failed to write .env file");
        true
    } else {
        false
    }
}
