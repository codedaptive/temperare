//! `next_step`: the text after `next: ` for an exit code.

use std::path::Path;

/// What the user does next for each exit code: 0 has nothing to do; 1 fixes the findings or adds an exception to
/// `rule_file`; 2 gives the error's fix ("see the error above" when there is none); 3 installs the engines named in
/// `not_available`, in the order given; 4 raises the security findings; 5 reads the engine output; any other code
/// points at the output above. Each argument is read only for the code that uses it.
pub fn next_step(exit: u8, fix: Option<&str>, rule_file: &Path, not_available: &[&str]) -> String {
    match exit {
        0 => "nothing to do".to_string(),
        1 => format!("fix the findings above, or add an approved [[exception]] to {}", rule_file.display()),
        2 => fix.unwrap_or("see the error above").to_string(),
        3 if not_available.is_empty() => {
            "install the engines that are not available (or run without those rules)".to_string()
        }
        3 => format!("install {} (or run without those rules)", not_available.join(", ")),
        4 => "raise the security findings with their owner; work goes on".to_string(),
        5 => "see the engine output above".to_string(),
        _ => "see the output above".to_string(),
    }
}
