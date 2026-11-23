//! `detekt_rule_list`: detekt's built-in rules as temperare lists them.

/// Each `(rule set, rule)` of `detekt_rules.txt`, in file order: detekt 1.23.8's 214 built-in rules, sorted by rule
/// set and then rule. The file's `#` lines and blank lines are not rules.
pub(crate) fn detekt_rule_list() -> Vec<(&'static str, &'static str)> {
    include_str!("detekt_rules.txt")
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .filter_map(|line| line.split_once('.'))
        .collect()
}
