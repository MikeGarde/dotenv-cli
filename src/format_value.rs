/// Normalise newlines for output.
///
/// multiline=false (default): convert actual newlines → literal `\n` sequences.
/// multiline=true  (--multiline flag): expand literal `\n` sequences → actual newlines.
pub fn format_value(value: &str, multiline: bool) -> String {
    if multiline {
        value.replace("\\n", "\n")
    } else {
        value.replace('\n', "\\n")
    }
}
