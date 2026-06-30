use dotenv_cli::qualifying_rules::{qualifying_rules, Options};

fn options() -> Options {
    Options {
        full_env_path: ".env".to_string(),
        env_object: None,
        json: false,
        no_json: false,
        multiline: false,
        action_set: false,
        action_delete: false,
        allow_missing: false,
        single_key: false,
        return_all_keys: false,
        target_keys: Vec::new(),
        set_value: None,
        debug: false,
    }
}

fn error_message(opts: Options) -> String {
    qualifying_rules(&opts).unwrap_err().to_string()
}

#[test]
fn allows_read_flags_without_set_or_delete() {
    let mut opts = options();
    opts.json = true;
    opts.no_json = true;
    opts.multiline = true;
    opts.return_all_keys = true;

    assert!(qualifying_rules(&opts).is_ok());
}

#[test]
fn allows_set_with_a_single_key() {
    let mut opts = options();
    opts.action_set = true;
    opts.single_key = true;

    assert!(qualifying_rules(&opts).is_ok());
}

#[test]
fn rejects_json_with_set() {
    let mut opts = options();
    opts.json = true;
    opts.action_set = true;
    opts.single_key = true;

    assert_eq!(error_message(opts), "Cannot use --json and --set together");
}

#[test]
fn rejects_set_without_a_single_key() {
    let mut opts = options();
    opts.action_set = true;

    assert_eq!(
        error_message(opts),
        "Must specify a single key when using --set"
    );
}

#[test]
fn allows_delete_with_a_single_key() {
    let mut opts = options();
    opts.action_delete = true;
    opts.single_key = true;

    assert!(qualifying_rules(&opts).is_ok());
}

#[test]
fn rejects_delete_with_set() {
    let mut opts = options();
    opts.action_delete = true;
    opts.action_set = true;
    opts.single_key = true;

    assert_eq!(
        error_message(opts),
        "Cannot use --delete with any other options"
    );
}

#[test]
fn rejects_delete_with_json() {
    let mut opts = options();
    opts.action_delete = true;
    opts.json = true;
    opts.single_key = true;

    assert_eq!(
        error_message(opts),
        "Cannot use --delete with any other options"
    );
}

#[test]
fn rejects_delete_with_multiline() {
    let mut opts = options();
    opts.action_delete = true;
    opts.multiline = true;
    opts.single_key = true;

    assert_eq!(
        error_message(opts),
        "Cannot use --delete with any other options"
    );
}

#[test]
fn rejects_delete_without_a_single_key() {
    let mut opts = options();
    opts.action_delete = true;

    assert_eq!(
        error_message(opts),
        "Must specify a single key when using --delete"
    );
}
