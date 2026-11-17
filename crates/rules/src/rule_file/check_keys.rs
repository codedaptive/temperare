//! `check_keys`: unknown keys in one table of the rule file.

use super::problem_at::problem_at;
use crate::Problem;
use toml::de::DeTable;

/// One Problem at each key of `table` not in `allowed`; `place` names the table (e.g. "[[rule]]").
pub(crate) fn check_keys(text: &str, table: &DeTable, allowed: &[&str], place: &str, out: &mut Vec<Problem>) {
    let fix = format!("Remove it, or use one of: {}.", allowed.join(", "));
    out.extend(
        table
            .iter()
            .map(|(key, _)| key)
            .filter(|key| !allowed.contains(&key.get_ref().as_ref()))
            .map(|key| problem_at(text, key.span().start, format!("unknown key `{}` in {place}", key.get_ref()), &fix)),
    );
}
