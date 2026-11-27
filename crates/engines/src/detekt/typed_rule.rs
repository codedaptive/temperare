//! `typed_rule`: whether a detekt rule needs type resolution.

/// True when `rule`, a rule name (`Deprecation`), is one of the rules `detekt_typed_rules.txt` lists: detekt 1.23.8's
/// rules that need type resolution, which cctao cannot give them (no `--classpath`), so they report nothing. A rule-set
/// name, or a name with its set (`potential-bugs.Deprecation`), is false. The file's `#` lines and blank lines are
/// not rules.
pub(crate) fn typed_rule(rule: &str) -> bool {
    include_str!("detekt_typed_rules.txt")
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .filter_map(|line| line.split_once('.'))
        .any(|(_, typed)| typed == rule)
}
