use crate::format_value::format_value;
use crate::qualifying_rules::Options;

/// Read and print one or more values from the parsed EnvObject.
///
/// Returns `true` if every requested key was found, `false` otherwise
/// (caller should set exit code 1).
///
/// JSON mode builds a proper JSON object via serde_json for correct escaping.
/// Plain mode prints one value per line.
pub fn get_value(options: &Options) -> bool {
    let env_object = options.env_object.as_ref().unwrap();
    let mut all_found = true;

    let keys: Vec<String> = if options.return_all_keys {
        env_object.keys().cloned().collect()
    } else {
        options.target_keys.clone()
    };

    if options.json {
        let mut map = serde_json::Map::new();
        for key in &keys {
            if let Some(env_val) = env_object.get(key) {
                let value = format_value(&env_val.value, options.multiline);
                map.insert(key.clone(), serde_json::Value::String(value));
            } else {
                all_found = false;
                map.insert(key.clone(), serde_json::Value::Null);
            }
        }
        println!(
            "{}",
            serde_json::to_string(&serde_json::Value::Object(map)).unwrap()
        );
    } else {
        let mut lines: Vec<String> = Vec::new();
        for key in &keys {
            if let Some(env_val) = env_object.get(key) {
                lines.push(format_value(&env_val.value, options.multiline));
            } else {
                all_found = false;
                lines.push(String::new()); // empty line for missing key, mirrors TS
            }
        }
        // Join and print — mirrors TS result.slice(0, -1) which trims trailing newline
        print!("{}", lines.join("\n"));
        println!(); // final newline
    }

    all_found
}
