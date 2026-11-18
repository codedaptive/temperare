//! `check_security_owner`: a rule file with a security rule must name a top-level owner.

use super::problem_at::problem_at;
use crate::Problem;
use toml::de::DeTable;

/// One Problem at the `kind` value of the first `[[rule]]` with `kind = "security"` when `owner` (the usable
/// top-level owner) is None. It reads the raw `[[rule]]` tables, so a security rule with other problems counts too.
pub(crate) fn check_security_owner(text: &str, root: &DeTable, owner: Option<&str>, out: &mut Vec<Problem>) {
    if owner.is_some() {
        return;
    }
    let Some(tables) = root.get("rule").and_then(|value| value.get_ref().as_array()) else {
        return;
    };
    let first = tables
        .iter()
        .filter_map(|item| item.get_ref().as_table()?.get("kind"))
        .find(|kind| kind.get_ref().as_str() == Some("security"));
    if let Some(kind) = first {
        out.push(problem_at(
            text,
            kind.span().start,
            "a security rule needs a top-level `owner`, and the rule file names none".to_string(),
            "Add a top-level `owner = \"<kit>\"`: a security finding must name its owner.",
        ));
    }
}
