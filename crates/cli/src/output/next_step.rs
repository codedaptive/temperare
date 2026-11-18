//! `next_step`: the text after `next: ` for an exit code.

use std::path::Path;

/// Tessera's checks never run yet (S-20): every tessera rule is not available whether or not `ts` is installed, so
/// exit 3 never tells the user to install it.
const TESSERA: &str = "tessera";
const TESSERA_NOT_YET: &str = "Tessera's checks are not available yet";

/// What the user does next for each exit code: 0 has nothing to do; 1 fixes the findings or adds an exception to
/// `rule_file`; 2 gives the error's fix ("see the error above" when there is none); 3 installs the engines named in
/// `not_available`, in the order given, and says Tessera's checks are not available yet when "tessera" is among them
/// rather than asking for it to be installed; 4 raises the security findings; 5 reads the engine output; any other
/// code points at the output above. Each argument is read only for the code that uses it.
pub fn next_step(exit: u8, fix: Option<&str>, rule_file: &Path, not_available: &[&str]) -> String {
    match exit {
        0 => "nothing to do".to_string(),
        1 => format!("fix the findings above, or add an approved [[exception]] to {}", rule_file.display()),
        2 => fix.unwrap_or("see the error above").to_string(),
        3 => not_available_step(not_available),
        4 => "raise the security findings with their owner; work goes on".to_string(),
        5 => "see the engine output above".to_string(),
        _ => "see the output above".to_string(),
    }
}

/// Exit 3's text: the engines to install, with tessera split out because installing it changes nothing.
fn not_available_step(not_available: &[&str]) -> String {
    if not_available.is_empty() {
        return "install the engines that are not available (or run without those rules)".to_string();
    }
    let others: Vec<&str> = not_available.iter().copied().filter(|name| *name != TESSERA).collect();
    if others.is_empty() {
        return format!("{TESSERA_NOT_YET} (run without those rules)");
    }
    if others.len() == not_available.len() {
        return format!("install {} (or run without those rules)", others.join(", "));
    }
    format!("install {}; {TESSERA_NOT_YET} (or run without those rules)", others.join(", "))
}
