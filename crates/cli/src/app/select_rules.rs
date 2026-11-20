//! `select_rules`: the rule file cut to the rules of one kind for `tm check --kind`.

use rules::{Kind, RuleFile};

/// `rf` with only its rules of `kind`, in file order; with no kind, `rf` whole. Every other field (the folder, kits,
/// owner and exceptions) is kept, so the report still counts every exception in force.
pub(crate) fn select_rules(rf: &RuleFile, kind: Option<Kind>) -> RuleFile {
    match kind {
        None => rf.clone(),
        Some(kind) => {
            let mut cut = rf.clone();
            cut.rules = rf.rules_of_kind(kind).into_iter().cloned().collect();
            cut
        }
    }
}
