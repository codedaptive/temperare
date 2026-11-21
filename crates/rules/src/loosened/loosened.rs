//! `loosened`: what a change to a rule file loosened.

use super::Loosening;
use super::exception_loosenings::exception_loosenings;
use super::rule_loosenings::rule_loosenings;
use super::set_loosenings::set_loosenings;
use crate::RuleFile;

/// The ways `now` is looser than `base` that this compares. It compares two `RuleFile`s with their taken sets resolved; it reads no
/// file and runs nothing.
///
/// For each rule in force at the base (`not_applicable` None), matched by id:
/// - RuleGone: now has it in neither `rules` nor `not_applicable_rules`. A rule an `[[override]]` turns off is left
///   out of the `RuleFile` entirely, so off and gone are one kind; so is a rule a newer set retired.
/// - RuleNotApplicable: now has it in `not_applicable_rules`, with its reason.
/// - SeverityLowered: now's severity is lower (error to warning).
/// - Paths narrowed: `paths` empty covers every file. Paths are narrowed when a pattern written in the base's `paths`
///   is missing from now's non-empty `paths` (PathRemoved, one per pattern), or when the base's `paths` is empty and
///   now's is not (PathsLimited). Excludes are widened when a pattern written in now's `exclude` is missing from the
///   base's (ExcludeAdded, one per pattern). Patterns are compared as written, so a rewritten pattern is one removed
///   and one added.
/// - Kits narrowed: `kits` empty sets no kit limit. Kits are narrowed when a kit written in the base's `kits` is
///   missing from now's non-empty `kits` (KitRemoved, one per kit), or when the base's `kits` is empty and now's is
///   not (KitsLimited). Outside kits are widened when a kit written in now's `outside_kits` is missing from the base's
///   (OutsideKitAdded, one per kit).
///
/// Rules not applicable at the base, and rules only now has, give nothing. Then for exceptions, at most one each: a
/// base exception covers one now when it has the same rule and path and its `what` is None or the same, so dropping a
/// `what` widens and adding one narrows. ExceptionAdded: an exception now that no base exception covers. For a covered
/// one, the base's date is the latest `expires` of its covering exceptions when all have one: ExpiryLater when now's
/// `expires` is later; ExpiryDropped when now has none. Then for baseline sets
/// ("name@version"): SetDropped when now takes no set of the name; SetOlder when now's version is lower, both read
/// as u32. A dropped set's rules are still reported RuleGone.
///
/// The order: the rule loosenings, in the base's `rules_in_order` and per rule in the order above (severity, removed
/// paths in the base's order, limited paths, added excludes in now's order, removed kits in the base's order, limited
/// kits, added outside kits in now's order); then the exceptions in now's file order; then the baseline entries in
/// the base's order. Tightenings give nothing. Changes to `params`, `check`, `kind`, `engine` and `only_with_base`
/// are not compared, nor are an exception's reason and approval.
pub fn loosened(base: &RuleFile, now: &RuleFile) -> Vec<Loosening> {
    let mut found = rule_loosenings(base, now);
    found.extend(exception_loosenings(base, now));
    found.extend(set_loosenings(base, now));
    found
}
