//! `ResolvedBaseline`: the baseline sets a rule file takes, loaded and changed by its overrides.

use crate::Rule;
use std::collections::BTreeSet;

/// What the taken sets give the rule file.
#[derive(Debug, Default)]
pub(crate) struct ResolvedBaseline {
    /// The taken sets as "name@version", in file order.
    pub(crate) taken: Vec<String>,
    /// The set rules that apply, in taken order and then each set file's order; off rules are left out.
    pub(crate) rules: Vec<Rule>,
    /// The set rules whose required params are not given, each with its reason in `rule.not_applicable`.
    pub(crate) not_applicable: Vec<Rule>,
    /// The ids the taken sets list as retired, in taken order.
    pub(crate) retired: Vec<String>,
    /// Every rule id of the taken sets, off and not-applicable rules included, so an exception may name any of them.
    pub(crate) ids: BTreeSet<String>,
    /// The byte offset of the `baseline` entry string that brings the first security rule in force; None when no
    /// rule in force is a security rule.
    pub(crate) security_at: Option<usize>,
}
