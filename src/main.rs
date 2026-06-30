#!/usr/bin/env -S cargo +nightly script
mod cli;
mod env_object;
mod env_parser;
mod errors;
mod format_value;
mod handlers;
mod qualifying_rules;
mod read_pipe;

use std::path::{Path, PathBuf};

use clap::Parser;

use cli::Cli;
use env_parser::parse_env_file;
use errors::{EnvParseError, RuleViolationError};
use qualifying_rules::{qualifying_rules, Options};

fn main() {
    let exit_code = match run() {
        Ok(code) => code,
        Err(e) => {
            if let Some(err) = e.downcast_ref::<RuleViolationError>() {
                eprintln!("{}", err);
            } else if let Some(err) = e.downcast_ref::<EnvParseError>() {
                // Include "EnvParseError" in stderr so tests can detect it
                eprintln!("An unexpected error occurred: EnvParseError: {}", err);
            } else {
                eprintln!("An unexpected error occurred: {}", e);
            }
            1
        }
    };
    std::process::exit(exit_code);
}

fn run() -> Result<i32, Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.version {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return Ok(0);
    }

    let debug = cli.debug;
    let multiline = cli.multiline;
    let delete = cli.delete;
    let keys = cli.key.clone();

    // `--set -` is the explicit opt-in to read the value from stdin. Without it,
    // stdin is never touched, so reading a key can never consume a caller's stdin.
    let set_value_raw = match cli.set.clone() {
        Some(v) if v == "-" => Some(read_pipe::read_stdin()),
        other => other,
    };
    let is_set = set_value_raw.is_some();

    let env_file = cli
        .file
        .or_else(|| std::env::var("DOTENV_FILE").ok())
        .unwrap_or_else(|| ".env".to_string());

    // Resolve to absolute path (mirrors Node's path.resolve)
    let full_env_path: PathBuf = if Path::new(&env_file).is_absolute() {
        PathBuf::from(&env_file)
    } else {
        std::env::current_dir()
            .map_err(|e| Box::new(RuleViolationError(e.to_string())) as Box<dyn std::error::Error>)?
            .join(&env_file)
    };
    let full_env_path_str = full_env_path.to_string_lossy().to_string();

    if debug {
        eprintln!("Keys: {:?}", keys);
        eprintln!("File: {}", full_env_path_str);
    }

    if !full_env_path.exists() {
        return Err(Box::new(RuleViolationError(format!(
            "File not found: {}",
            full_env_path_str
        ))));
    }

    let mut options = Options {
        full_env_path: full_env_path_str,
        env_object: None,
        json: cli.json,
        no_json: cli.no_json,
        multiline,
        action_set: is_set,
        action_delete: delete,
        allow_missing: cli.allow_missing,
        single_key: keys.len() == 1,
        return_all_keys: keys.is_empty(),
        target_keys: keys.clone(),
        set_value: set_value_raw,
        debug,
    };

    // Multiple keys or no keys default to JSON output (unless overridden with --no-json)
    if (keys.len() > 1 || keys.is_empty()) && !options.no_json {
        if debug {
            eprintln!("Key count (0 or >1) defaulting to JSON");
        }
        options.json = true;
    }

    if debug {
        eprintln!("Options assembled");
    }

    qualifying_rules(&options).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    let env_object = parse_env_file(&options.full_env_path)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    // Wildcard expansion: `DB_*` → matched keys, automatically enables JSON output
    if keys.len() == 1 && !options.no_json && options.target_keys[0].contains('*') {
        if debug {
            eprintln!("Wildcard found");
        }
        let pattern = options.target_keys[0].clone();
        let matched: Vec<String> = env_object
            .keys()
            .filter(|k| wildcard_match(&pattern, k))
            .cloned()
            .collect();
        options.target_keys = matched;
        options.json = true;
    }

    options.env_object = Some(env_object);

    // Route to the appropriate handler
    let exit_code = if options.json && options.return_all_keys {
        println!(
            "{}",
            options
                .env_object
                .as_ref()
                .unwrap()
                .to_json_string(options.multiline)
        );
        0
    } else if options.action_delete {
        let found = handlers::delete_key::delete_key(&options);
        if !found {
            1
        } else {
            0
        }
    } else if options.action_set {
        handlers::set_value::set_value(&options);
        0
    } else {
        let all_found = handlers::get_value::get_value(&options);
        if all_found || options.allow_missing {
            0
        } else {
            1
        }
    };

    Ok(exit_code)
}

/// Simple glob wildcard matching: `*` matches any sequence of characters.
/// `DB_*` matches `DB_HOST`, `DB_USER`, etc.
fn wildcard_match(pattern: &str, text: &str) -> bool {
    let parts: Vec<&str> = pattern.split('*').collect();
    if parts.len() == 1 {
        return pattern == text;
    }

    let mut remaining = text;
    for (i, part) in parts.iter().enumerate() {
        if i == 0 {
            if !remaining.starts_with(part) {
                return false;
            }
            remaining = &remaining[part.len()..];
        } else if i == parts.len() - 1 {
            if !remaining.ends_with(part) {
                return false;
            }
        } else {
            match remaining.find(part) {
                Some(pos) => remaining = &remaining[pos + part.len()..],
                None => return false,
            }
        }
    }
    true
}
