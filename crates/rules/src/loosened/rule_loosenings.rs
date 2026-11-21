//! `rule_loosenings`: the rules a change took out of force or loosened.

use super::{Loosening, LooseningKind};
use crate::{Rule, RuleFile, Severity};

/// For each rule in force at the base, in the base's `rules_in_order`, matched by id: RuleGone when now has no rule
/// of that id; RuleNotApplicable when now's is not applicable; otherwise what `changed_rule` finds. Rules not
/// applicable at the base, and rules only now has, give nothing.
pub(super) fn rule_loosenings(base: &RuleFile, now: &RuleFile) -> Vec<Loosening> {
    let mut found = Vec::new();
    for rule in base.rules_in_order().into_iter().filter(|rule| rule.not_applicable.is_none()) {
        if let Some(later) = now.rules.iter().find(|later| later.id == rule.id) {
            changed_rule(rule, later, &mut found);
        } else if let Some(later) = now.not_applicable_rules.iter().find(|later| later.id == rule.id) {
            found.push(loosening(LooseningKind::RuleNotApplicable, rule, None, later.not_applicable.clone()));
        } else {
            found.push(loosening(LooseningKind::RuleGone, rule, None, None));
        }
    }
    found
}

/// A rule still in force: SeverityLowered, then a PathRemoved per base pattern missing from now's non-empty `paths`,
/// then PathsLimited when `paths` went from empty to not, then an ExcludeAdded per new exclude pattern; then the same
/// for kits: a KitRemoved per base kit missing from now's non-empty `kits`, KitsLimited when `kits` went from empty
/// to not, and an OutsideKitAdded per new `outside_kits` kit.
fn changed_rule(base: &Rule, now: &Rule, found: &mut Vec<Loosening>) {
    if rank(now.severity) < rank(base.severity) {
        let (before, after) = (name(base.severity), name(now.severity));
        found.push(loosening(LooseningKind::SeverityLowered, base, Some(before.to_string()), Some(after.to_string())));
    }
    if !now.paths.is_empty() {
        for pattern in base.paths.iter().filter(|pattern| !now.paths.contains(pattern)) {
            found.push(loosening(LooseningKind::PathRemoved, base, Some(pattern.clone()), None));
        }
        if base.paths.is_empty() {
            found.push(loosening(LooseningKind::PathsLimited, base, None, Some(now.paths.join(", "))));
        }
    }
    for pattern in now.exclude.iter().filter(|pattern| !base.exclude.contains(pattern)) {
        found.push(loosening(LooseningKind::ExcludeAdded, base, None, Some(pattern.clone())));
    }
    if !now.kits.is_empty() {
        for kit in base.kits.iter().filter(|kit| !now.kits.contains(kit)) {
            found.push(loosening(LooseningKind::KitRemoved, base, Some(kit.clone()), None));
        }
        if base.kits.is_empty() {
            found.push(loosening(LooseningKind::KitsLimited, base, None, Some(now.kits.join(", "))));
        }
    }
    for kit in now.outside_kits.iter().filter(|kit| !base.outside_kits.contains(kit)) {
        found.push(loosening(LooseningKind::OutsideKitAdded, base, None, Some(kit.clone())));
    }
}

fn loosening(kind: LooseningKind, rule: &Rule, before: Option<String>, after: Option<String>) -> Loosening {
    Loosening { kind, about: rule.id.clone(), before, after }
}

/// Severity has no order of its own; a higher rank counts for more.
fn rank(severity: Severity) -> u8 {
    match severity {
        Severity::Error => 2,
        Severity::Warning => 1,
    }
}

/// The severity's rule-file name, as its serde rename gives it.
fn name(severity: Severity) -> &'static str {
    match severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
    }
}
