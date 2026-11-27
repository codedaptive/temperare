//! Unit tests for loosening_kind.rs: each variant serializes to its report name.

use super::LooseningKind;
use serde::Serialize;

#[derive(Serialize)]
struct Wrap {
    kind: LooseningKind,
}

#[test]
fn every_variant_serializes_to_its_name() {
    let rows = [
        (LooseningKind::RuleGone, "rule_gone"),
        (LooseningKind::RuleNotApplicable, "rule_not_applicable"),
        (LooseningKind::SeverityLowered, "severity_lowered"),
        (LooseningKind::PathRemoved, "path_removed"),
        (LooseningKind::PathsLimited, "paths_limited"),
        (LooseningKind::ExcludeAdded, "exclude_added"),
        (LooseningKind::KitRemoved, "kit_removed"),
        (LooseningKind::KitsLimited, "kits_limited"),
        (LooseningKind::OutsideKitAdded, "outside_kit_added"),
        (LooseningKind::ExceptionAdded, "exception_added"),
        (LooseningKind::ExpiryLater, "expiry_later"),
        (LooseningKind::SetDropped, "set_dropped"),
        (LooseningKind::SetOlder, "set_older"),
    ];
    for (kind, name) in rows {
        assert_eq!(toml::to_string(&Wrap { kind }).unwrap(), format!("kind = \"{name}\"\n"), "{kind:?}");
    }
}
