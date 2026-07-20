use std::process::Command;

use crate::env_object::EnvObject;

pub fn run_command(env_object: &EnvObject, command: &[String], debug: bool) -> i32 {
    let program = &command[0];
    let args = &command[1..];

    let mut cmd = Command::new(program);
    cmd.args(args);

    for (key, env_val) in env_object.entries.iter() {
        // Don't override variables already present in the environment.
        if std::env::var_os(key).is_none() {
            cmd.env(key, &env_val.value);
        } else if debug {
            eprintln!("Skipping {} (already set in the environment)", key);
        }
    }

    if debug {
        eprintln!("Running: {} {}", program, args.join(" "));
    }

    match cmd.status() {
        Ok(status) => status.code().unwrap_or_else(|| {
            // Terminated by a signal (Unix): mirror shells with 128 + signal.
            #[cfg(unix)]
            {
                use std::os::unix::process::ExitStatusExt;
                status.signal().map(|s| 128 + s).unwrap_or(1)
            }
            #[cfg(not(unix))]
            {
                1
            }
        }),
        Err(e) => {
            eprintln!("dotenv: failed to run '{}': {}", program, e);
            127
        }
    }
}
