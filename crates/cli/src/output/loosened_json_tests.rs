//! ZUT for loosened_json.rs: the object for literal loosenings of all fourteen kinds, absent before and after left
//! out. RuleNotApplicable and SetOlder are covered only here and in loosened_text_tests.rs.

use super::loosened_json;
use super::loosened_text_tests::every_kind;
use serde_json::json;
use std::path::Path;

#[test]
fn every_kind_with_only_the_values_it_has() {
    let value = loosened_json(Path::new("/repo/temperare.toml"), "main", &every_kind());
    assert_eq!(
        value,
        json!({
            "rule_file": "/repo/temperare.toml",
            "base": "main",
            "count": 14,
            "loosened": [
                {"kind": "rule_gone", "about": "r1"},
                {"kind": "rule_not_applicable", "about": "r2", "after": "needs param max"},
                {"kind": "severity_lowered", "about": "r3", "before": "error", "after": "warning"},
                {"kind": "path_removed", "about": "r4", "before": "src/**"},
                {"kind": "paths_limited", "about": "r5", "after": "a/**, b/**"},
                {"kind": "exclude_added", "about": "r6", "after": "gen/**"},
                {"kind": "kit_removed", "about": "r7", "before": "core"},
                {"kind": "kits_limited", "about": "r8", "after": "core, cli"},
                {"kind": "outside_kit_added", "about": "r9", "after": "vendor"},
                {"kind": "exception_added", "about": "r10", "after": "old.rs (E1)"},
                {"kind": "expiry_later", "about": "r11", "before": "2026-01-01", "after": "2027-01-01"},
                {"kind": "expiry_dropped", "about": "r12", "before": "2026-02-03"},
                {"kind": "set_dropped", "about": "standard", "before": "1"},
                {"kind": "set_older", "about": "house", "before": "2", "after": "1"},
            ],
        })
    );
}
