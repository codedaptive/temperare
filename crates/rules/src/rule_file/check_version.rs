//! `check_version`: the rule file's `version`.

use super::problem_at::problem_at;
use crate::Problem;
use toml::de::DeTable;

/// A Problem when `version` is absent (at 1:1) or is not the integer 1 (at the value).
pub(crate) fn check_version(text: &str, root: &DeTable, out: &mut Vec<Problem>) {
    let Some(value) = root.get("version") else {
        out.push(problem_at(text, 0, "`version` is missing".to_string(), "Add `version = 1` as the first line."));
        return;
    };
    let is_one = value
        .get_ref()
        .as_integer()
        .is_some_and(|i| i64::from_str_radix(i.as_str(), i.radix()) == Ok(1));
    if !is_one {
        out.push(problem_at(
            text,
            value.span().start,
            format!("`version` must be 1, not {}", text.get(value.span()).unwrap_or_default()),
            "Write `version = 1`.",
        ));
    }
}
