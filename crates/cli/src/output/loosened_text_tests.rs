//! ZUT for loosened_text.rs: the exact lines for literal loosenings of thirteen kinds, all but a dropped expiry (see
//! `every_kind`).
//! RuleNotApplicable also comes from a real rule file and is covered by the RIT `rules_loosened_rule_not_applicable`.
//! SetOlder is covered only by ZUTs (these, and rules' own set_loosenings_tests.rs), because only version 1 of each
//! set exists.

use super::loosened_text;
use rules::{Loosening, LooseningKind as K};
use std::path::Path;

fn l(kind: K, about: &str, before: Option<&str>, after: Option<&str>) -> Loosening {
    Loosening { kind, about: about.to_string(), before: before.map(str::to_string), after: after.map(str::to_string) }
}

/// One loosening of each kind but a dropped expiry, with before and after as each kind has them; loosened_json_tests.rs
/// reads it too. A dropped expiry is left out: only an exception with no expires date can give one.
pub(super) fn every_kind() -> Vec<Loosening> {
    vec![
        l(K::RuleGone, "r1", None, None),
        l(K::RuleNotApplicable, "r2", None, Some("needs param max")),
        l(K::SeverityLowered, "r3", Some("error"), Some("warning")),
        l(K::PathRemoved, "r4", Some("src/**"), None),
        l(K::PathsLimited, "r5", None, Some("a/**, b/**")),
        l(K::ExcludeAdded, "r6", None, Some("gen/**")),
        l(K::KitRemoved, "r7", Some("core"), None),
        l(K::KitsLimited, "r8", None, Some("core, cli")),
        l(K::OutsideKitAdded, "r9", None, Some("vendor")),
        l(K::ExceptionAdded, "r10", None, Some("old.rs (E1)")),
        l(K::ExpiryLater, "r11", Some("2026-01-01"), Some("2027-01-01")),
        l(K::SetDropped, "standard", Some("1"), None),
        l(K::SetOlder, "house", Some("2"), Some("1")),
    ]
}

#[test]
fn every_kind_one_line_each_in_order() {
    let text = loosened_text(Path::new("/repo/tao.toml"), "main", &every_kind());
    assert_eq!(
        text,
        "rule file: /repo/tao.toml\n\
         base: main\n\
         loosened (13):\n  \
         rule_gone  r1\n  \
         rule_not_applicable  r2  after: needs param max\n  \
         severity_lowered  r3  before: error  after: warning\n  \
         path_removed  r4  before: src/**\n  \
         paths_limited  r5  after: a/**, b/**\n  \
         exclude_added  r6  after: gen/**\n  \
         kit_removed  r7  before: core\n  \
         kits_limited  r8  after: core, cli\n  \
         outside_kit_added  r9  after: vendor\n  \
         exception_added  r10  after: old.rs (E1)\n  \
         expiry_later  r11  before: 2026-01-01  after: 2027-01-01\n  \
         set_dropped  standard  before: 1\n  \
         set_older  house  before: 2  after: 1\n"
    );
}
