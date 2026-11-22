//! `out_of_play`: the rules the report lists as covering nothing or not applicable, and the ids it drops.

use crate::{set_aside_why, CoversNothing, NotApplicable};
use rules::{RuleFile, ONLY_WITH_BASE_WHY};

/// The start of every baseline set's rule id; a repository's own ids never start with it (rules' `check_id`).
const BASELINE_PREFIX: &str = "tm-";

/// The rules that cover nothing, the rules not applied, and the ids of the rules of `rules.rules` among them.
///
/// One walk of `RuleFile::rules_in_order` (by `position`; rules sharing one keep `rules` first, then
/// `not_applicable_rules`), so both lists are in rule-file order and a rule is listed once, by the first of these that
/// holds: a rule that sits in `rules.not_applicable_rules`, told by its address in that list and not by its reason, is
/// not applicable with `set_aside_why` (its own reason, "no reason given" when None); else, when `no_base`, a rule with
/// `only_with_base` is not applicable with `rules::ONLY_WITH_BASE_WHY`; else a rule flagged in `no_files` (rule id,
/// why; the first pair for an id counts) is not applicable with that why when its id starts "tm-" (a baseline set's),
/// and otherwise (the repository's own) covers nothing. A flagged id that names no rule of the file is ignored: there
/// is no engine or rule-file place to list it with. The third list holds the ids of the listed rules of `rules.rules`,
/// in the same order: the rules whose outcomes the report drops. A set-aside rule is never in it. Pure.
pub fn out_of_play<'a>(
    rules: &'a RuleFile,
    no_base: bool,
    no_files: &[(String, String)],
) -> (Vec<CoversNothing>, Vec<NotApplicable>, Vec<&'a str>) {
    let set_aside = rules.not_applicable_rules.as_ptr_range();
    let (mut covers_nothing, mut not_applicable, mut dropped) = (Vec::new(), Vec::new(), Vec::new());
    for rule in rules.rules_in_order() {
        let flag = no_files.iter().find(|(id, _)| *id == rule.id).map(|(_, why)| why.as_str());
        let listed = |why: &str| NotApplicable { rule: rule.id.clone(), engine: rule.engine, why: why.to_string() };
        match (set_aside.contains(&std::ptr::from_ref(rule)), no_base && rule.only_with_base, flag) {
            (true, _, _) => {
                not_applicable.push(listed(set_aside_why(rule)));
                continue;
            }
            (_, true, _) => not_applicable.push(listed(ONLY_WITH_BASE_WHY)),
            (_, _, Some(why)) if rule.id.starts_with(BASELINE_PREFIX) => not_applicable.push(listed(why)),
            (_, _, Some(_)) => covers_nothing.push(CoversNothing { rule: rule.id.clone(), engine: rule.engine }),
            (_, _, None) => continue,
        }
        dropped.push(rule.id.as_str());
    }
    (covers_nothing, not_applicable, dropped)
}
