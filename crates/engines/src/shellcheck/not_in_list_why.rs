//! `not_in_list_why`: the why of a rule whose check is not in temperare's list of its engine's codes.

use super::CodeList;

/// `<engine> <version>: <check> is not in temperare's list of <engine> codes (taken from <engine> <list version>)`,
/// with `(version unknown)` for the installed version when it was not read.
pub(crate) fn not_in_list_why(engine: &str, version: Option<&str>, check: &str, list: &CodeList) -> String {
    format!(
        "{engine} {}: {check} is not in temperare's list of {engine} codes (taken from {engine} {})",
        version.unwrap_or("(version unknown)"),
        list.version
    )
}
