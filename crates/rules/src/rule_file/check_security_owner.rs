//! `check_security_owner`: a rule file with a security rule in force must name a top-level owner.

use super::problem_at::problem_at;
use crate::Problem;
use toml::de::DeTable;

/// When `owner` (the usable top-level owner) is None, one Problem at most: at the `kind` value of the file's first
/// `[[rule]]` with `kind = "security"`, else at `baseline_security_at` (the `baseline` entry that brings the first
/// security rule in force) when it is Some. It reads the file's raw `[[rule]]` tables, so an own security rule with
/// other problems counts too.
pub(crate) fn check_security_owner(
    text: &str,
    root: &DeTable,
    owner: Option<&str>,
    baseline_security_at: Option<usize>,
    out: &mut Vec<Problem>,
) {
    if owner.is_some() {
        return;
    }
    let Some(at) = own_security_kind(root).or(baseline_security_at) else {
        return;
    };
    out.push(problem_at(
        text,
        at,
        "a security rule needs a top-level `owner`, and the rule file names none".to_string(),
        "Add a top-level `owner = \"<kit>\"`: a security finding must name its owner.",
    ));
}

/// The byte offset of the `kind` value of the first raw `[[rule]]` table with `kind = "security"`.
fn own_security_kind(root: &DeTable) -> Option<usize> {
    root.get("rule")?
        .get_ref()
        .as_array()?
        .iter()
        .filter_map(|item| item.get_ref().as_table()?.get("kind"))
        .find(|kind| kind.get_ref().as_str() == Some("security"))
        .map(|kind| kind.span().start)
}
