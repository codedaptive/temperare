//! `rules_not_applicable`: the rules `cctao rules` lists as not applicable, by `report::out_of_play`'s judgement with
//! no base.

use engines::NoFileRule;
use report::NotApplicable;
use rules::RuleFile;

/// The not-applicable list of `report::out_of_play` for `rf` with no base (`no_base` false) and `no_files` as its
/// flags, in rule-file order: a rule set aside in `rf.not_applicable_rules`, told by its
/// address, is listed with `report::set_aside_why`; else a baseline set's rule ("tm-") flagged in `no_files` is listed
/// with the first flag's why. A repository's own no-file rule is not listed (`cctao check` reports it as covering
/// nothing), and a flag naming no rule of `rf` is ignored. With no base, a rule that reports only what a change adds
/// is listed only through `no_files`. Pure.
pub(crate) fn rules_not_applicable(rf: &RuleFile, no_files: &[NoFileRule]) -> Vec<NotApplicable> {
    let pairs: Vec<(String, String)> = no_files.iter().map(|flag| (flag.rule.clone(), flag.why.clone())).collect();
    report::out_of_play(rf, false, &pairs).1
}
