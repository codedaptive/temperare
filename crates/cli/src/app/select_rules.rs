//! `select_rules`: the rule file cut to the rules of one kind for `tm check --kind`.

use rules::{Kind, RuleFile};

/// `rf` with only its rules of `kind`, in file order; with no kind, `rf` whole. The folder, kits, owner and
/// exceptions are kept, so the report still counts every exception in force.
pub(crate) fn select_rules(rf: &RuleFile, kind: Option<Kind>) -> RuleFile {
    match kind {
        None => rf.clone(),
        Some(kind) => RuleFile {
            folder: rf.folder.clone(),
            kits: rf.kits.clone(),
            owner: rf.owner.clone(),
            rules: rf.rules_of_kind(kind).into_iter().cloned().collect(),
            exceptions: rf.exceptions.clone(),
        },
    }
}
