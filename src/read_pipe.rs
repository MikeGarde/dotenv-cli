use is_terminal::IsTerminal;
use std::io::{self, Read};

/// Read all piped stdin and return it trimmed, or `None` when stdin is a TTY
/// (i.e. no pipe is present).
pub fn read_pipe() -> Option<String> {
    if io::stdin().is_terminal() {
        return None;
    }
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok()?;
    let trimmed = input.trim().to_string();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}
