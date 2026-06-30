use crate::env_object::EnvObject;
use crate::errors::RuleViolationError;

/// All resolved CLI options passed to the three action handlers.
#[allow(dead_code)]
pub struct Options {
    pub full_env_path: String,
    pub env_object: Option<EnvObject>,
    /// --json was passed explicitly
    pub json: bool,
    /// --no-json was passed explicitly
    pub no_json: bool,
    pub multiline: bool,
    pub action_set: bool,
    pub action_delete: bool,
    pub allow_missing: bool,
    pub single_key: bool,
    pub return_all_keys: bool,
    pub target_keys: Vec<String>,
    /// Literal value ready to write into the file
    pub set_value: Option<String>,
    pub debug: bool,
}

/// Validate option combinations (mirrors qualifyingRules.ts).
pub fn qualifying_rules(opts: &Options) -> Result<(), RuleViolationError> {
    if opts.json && opts.action_set {
        return Err(RuleViolationError(
            "Cannot use --json and --set together".to_string(),
        ));
    }
    if opts.action_set && !opts.single_key {
        return Err(RuleViolationError(
            "Must specify a single key when using --set".to_string(),
        ));
    }
    if opts.action_delete && (opts.action_set || opts.json || opts.multiline) {
        return Err(RuleViolationError(
            "Cannot use --delete with any other options".to_string(),
        ));
    }
    if opts.action_delete && !opts.single_key {
        return Err(RuleViolationError(
            "Must specify a single key when using --delete".to_string(),
        ));
    }
    Ok(())
}
