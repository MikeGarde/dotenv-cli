use indexmap::IndexMap;

/// Mirrors EnvValue in envObject.ts.
/// lineStart/lineEnd are 0-based line indices into the raw .env file.
/// -1 means the value was not parsed from a file (e.g. appended).
#[derive(Debug, Clone)]
pub struct EnvValue {
    pub value: String,
    pub line_start: i64,
    pub line_end: i64,
}

impl EnvValue {
    #[allow(dead_code)]
    pub fn new(value: String) -> Self {
        EnvValue {
            value,
            line_start: -1,
            line_end: -1,
        }
    }

    pub fn with_lines(value: String, line_start: i64, line_end: i64) -> Self {
        EnvValue {
            value,
            line_start,
            line_end,
        }
    }
}

/// Ordered map of env keys → EnvValue. Insertion order is preserved (IndexMap).
/// Iteration with .keys() / .iter() yields only env keys, not struct methods.
pub struct EnvObject {
    pub entries: IndexMap<String, EnvValue>,
}

impl Default for EnvObject {
    fn default() -> Self {
        Self::new()
    }
}

impl EnvObject {
    pub fn new() -> Self {
        EnvObject {
            entries: IndexMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&EnvValue> {
        self.entries.get(key)
    }

    pub fn set(&mut self, key: String, value: EnvValue) {
        self.entries.insert(key, value);
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.entries.keys()
    }

    /// Resolve ${VAR} references in-place. Iterates until stable (handles chains).
    pub fn resolve_nested_variables(&mut self) {
        let keys: Vec<String> = self.entries.keys().cloned().collect();
        for key in &keys {
            let value = self.entries[key.as_str()].value.clone();
            if !value.contains("${") {
                continue;
            }
            let resolved = self.resolve_value(&value);
            if resolved != value {
                self.entries[key.as_str()].value = resolved;
            }
        }
    }

    fn resolve_value(&self, input: &str) -> String {
        let mut result = input.to_string();
        loop {
            let prev = result.clone();
            let mut new_result = String::new();
            let mut chars = result.chars().peekable();
            while let Some(c) = chars.next() {
                if c == '$' && chars.peek() == Some(&'{') {
                    chars.next(); // consume '{'
                    let mut var_name = String::new();
                    loop {
                        match chars.next() {
                            Some('}') | None => break,
                            Some(ch) => var_name.push(ch),
                        }
                    }
                    if let Some(env_val) = self.entries.get(&var_name) {
                        new_result.push_str(&env_val.value);
                    } else {
                        new_result.push_str(&format!("${{{}}}", var_name));
                    }
                } else {
                    new_result.push(c);
                }
            }
            result = new_result;
            if result == prev {
                break;
            }
        }
        result
    }

    /// Serialise all keys to JSON. multiline=true → pretty-print (2-space indent).
    /// Values are stored raw; no newline conversion is performed here.
    pub fn to_json_string(&self, multiline: bool) -> String {
        let map: serde_json::Map<String, serde_json::Value> = self
            .entries
            .iter()
            .map(|(k, v)| (k.clone(), serde_json::Value::String(v.value.clone())))
            .collect();
        let val = serde_json::Value::Object(map);
        if multiline {
            serde_json::to_string_pretty(&val).unwrap()
        } else {
            serde_json::to_string(&val).unwrap()
        }
    }
}
