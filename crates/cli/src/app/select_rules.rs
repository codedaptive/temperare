//! `select_rules`: the rule file cut to the rules, and not-applicable rules, of one kind for `cctao check --kind`.

use rules::{Kind, RuleFile};

/// `rf` with its rules and its not-applicable rules both cut to `kind`, each in file order; with no kind, `rf` whole.
/// Every other field (folder, kits, owner, exceptions, baseline, retired) is kept, so the report still counts every
/// exception in force.
pub(crate) fn select_rules(rf: &RuleFile, kind: Option<Kind>) -> RuleFile {
    match kind {
        None => rf.clone(),
        Some(kind) => {
            let mut cut = rf.clone();
            cut.rules = rf.rules_of_kind(kind).into_iter().cloned().collect();
            cut.not_applicable_rules.retain(|rule| rule.kind == kind);
            cut
        }
    }
}
