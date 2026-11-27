//! `md_rule_of`: the markdownlint rule a check names, from cctao's list.

use super::{MdRule, MD_RULES};

/// The rule of `MD_RULES` one of whose names (the `MDnnn` id or an alias) equals `check`, ignoring ASCII case, as
/// markdownlint matches config keys; `None` when no rule has that name. A tag (`headings`) names a group of rules, not
/// one, so it is not a check and gives `None`.
pub(crate) fn md_rule_of(check: &str) -> Option<&'static MdRule> {
    MD_RULES.iter().find(|rule| rule.names.iter().any(|name| name.eq_ignore_ascii_case(check)))
}
