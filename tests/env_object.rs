use dotenv_cli::env_object::{EnvObject, EnvValue};

#[test]
fn resolve_single_variable() {
    let mut env = EnvObject::new();
    env.set(
        "VAR1".to_string(),
        EnvValue::with_lines("Hello".to_string(), 0, 0),
    );
    env.set(
        "VAR2".to_string(),
        EnvValue::with_lines("${VAR1} World".to_string(), 1, 1),
    );
    env.resolve_nested_variables();
    assert_eq!(env.get("VAR2").unwrap().value, "Hello World");
}

#[test]
fn resolve_two_variables() {
    let mut env = EnvObject::new();
    env.set(
        "VAR1".to_string(),
        EnvValue::with_lines("Hello".to_string(), 0, 0),
    );
    env.set(
        "VAR2".to_string(),
        EnvValue::with_lines("World".to_string(), 1, 1),
    );
    env.set(
        "VAR3".to_string(),
        EnvValue::with_lines("${VAR1} ${VAR2}".to_string(), 2, 2),
    );
    env.resolve_nested_variables();
    assert_eq!(env.get("VAR3").unwrap().value, "Hello World");
}

#[test]
fn resolve_merged_value() {
    let mut env = EnvObject::new();
    env.set(
        "VAR1".to_string(),
        EnvValue::with_lines("Hello".to_string(), 0, 0),
    );
    env.set(
        "VAR2".to_string(),
        EnvValue::with_lines("${VAR1} World".to_string(), 1, 1),
    );
    env.set(
        "VAR3".to_string(),
        EnvValue::with_lines("${VAR2} & Universe".to_string(), 2, 2),
    );
    env.resolve_nested_variables();
    assert_eq!(env.get("VAR3").unwrap().value, "Hello World & Universe");
}
