//! `MdRule`: one markdownlint rule as cctao's list holds it. Plain data, read by `md_rule_of` and
//! `check_off_why`.

/// One rule of markdownlint: its names, the `MDnnn` id first and then its aliases (`["MD013", "line-length"]`), and
/// its tags (`["line_length"]`). A config key that equals any of them, ignoring case, sets the rule; a check names it
/// by a name only, never by a tag.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct MdRule {
    pub(crate) names: &'static [&'static str],
    pub(crate) tags: &'static [&'static str],
}
