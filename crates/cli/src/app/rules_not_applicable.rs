//! `rules_not_applicable`: the rules `tm rules` lists as not applicable, judged as `tm check`'s report judges them,
//! except that a rule that reports only what a change adds is listed only when it has no file.

use engines::NoFileRule;
use report::{set_aside_why, NotApplicable};
use rules::RuleFile;

/// The start of every baseline set's rule id; a repository's own ids never start with it, and its own no-file rule is
/// one `tm check` reports as covering nothing, not as not applicable.
const BASELINE_PREFIX: &str = "tm-";

/// The rules of `rf` that are not applicable, in rule-file order (`RuleFile::rules_in_order`), each listed once by the
/// first of these that holds: a rule that sits in `rf.not_applicable_rules`, told by its address in that list and not
/// by its reason or id, is listed with `report::set_aside_why`; else a rule whose id starts "tm-" and has an entry in
/// `no_files` is listed with the first such entry's why. Any other rule, a repository's own no-file rule included, is
/// not listed, and an entry of `no_files` that names no rule of `rf` is ignored. There is no base here, so a rule
/// that reports only what a change adds is listed only through `no_files`, which carries it when it has no file. Pure:
/// it reads no disk and runs nothing.
pub(crate) fn rules_not_applicable(rf: &RuleFile, no_files: &[NoFileRule]) -> Vec<NotApplicable> {
    let set_aside = rf.not_applicable_rules.as_ptr_range();
    rf.rules_in_order()
        .into_iter()
        .filter_map(|rule| {
            let why = if set_aside.contains(&std::ptr::from_ref(rule)) {
                set_aside_why(rule).to_string()
            } else if rule.id.starts_with(BASELINE_PREFIX) {
                no_files.iter().find(|flag| flag.rule == rule.id)?.why.clone()
            } else {
                return None;
            };
            Some(NotApplicable { rule: rule.id.clone(), engine: rule.engine, why })
        })
        .collect()
}
