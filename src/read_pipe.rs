use std::io::{self, Read};

/// Read all of stdin and return it trimmed.
///
/// Called only when the user explicitly opts into stdin with `--set -`, so this
/// never inspects whether stdin is a TTY: reading is always an explicit request.
pub fn read_stdin() -> String {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read from stdin");
    input.trim().to_string()
}
