//! Unit tests for rules_json.rs: the object's fields, from rule files parsed from literal TOML (no disk) and engine
//! availability from closures (never the real PATH).

use super::rules_json;
use engines::Availability;
use rules::{Date, Engine};
use serde_json::json;
use std::path::Path;

fn all_available(_: Engine) -> Availability {
    Availability::Available
}

/// The day every listing here is taken on, 2026-11-21; the FULL exceptions expire long after it.
fn day() -> Date {
    Date::new(2026, 11, 21).unwrap()
}

fn parse(text: &str) -> rules::RuleFile {
    rules::RuleFile::parse(text, Path::new(".")).unwrap()
}

const FULL: &str = r#"version = 1
owner = "cli"

[kits.core]
paths = ["crates/core/**"]

[[rule]]
id = "lint-all"
engine = "clippy"
check = "clippy::all"
fix = "Do what clippy says."

[[rule]]
id = "no-unwrap"
engine = "clippy"
kind = "security"
check = "clippy::unwrap_used"
paths = ["src/**"]
exclude = ["**/tests/**"]
kits = ["core"]
severity = "warning"
fix = "Return a typed error."

[[exception]]
rule = "no-unwrap"
path = "src/a.rs"
what = "unwrap"
reason = "Legacy."
approved = "bob 2026-11-17"
expires = "2099-12-31"

[[exception]]
rule = "lint-all"
path = "src/b.rs"
reason = "Generated."
approved = "bob 2026-11-16"
expires = "2099-12-31"
"#;

#[test]
fn rules_exceptions_and_count_in_file_order() {
    assert_eq!(
        rules_json(Path::new("repo/temperare.toml"), &parse(FULL), &all_available, day()),
        json!({
            "rule_file": "repo/temperare.toml",
            "rules": [
                {"id": "lint-all", "engine": "clippy", "kind": "quality", "severity": "error",
                 "check": "clippy::all", "paths": [], "exclude": [], "kits": [], "outside_kits": [],
                 "available": true, "why": null},
                {"id": "no-unwrap", "engine": "clippy", "kind": "security", "severity": "warning",
                 "check": "clippy::unwrap_used", "paths": ["src/**"], "exclude": ["**/tests/**"], "kits": ["core"],
                 "outside_kits": [], "available": true, "why": null},
            ],
            "exceptions": [
                {"rule": "no-unwrap", "path": "src/a.rs", "what": "unwrap", "reason": "Legacy.",
                 "approved": "bob 2026-11-17"},
                {"rule": "lint-all", "path": "src/b.rs", "reason": "Generated.",
                 "approved": "bob 2026-11-16"},
            ],
            "exceptions_in_force": 2,
            "expired": [],
            "exceptions_expired": 0,
        })
    );
}

#[test]
fn an_empty_file_has_empty_lists_and_zero_in_force() {
    assert_eq!(
        rules_json(Path::new("t.toml"), &parse("version = 1\n"), &all_available, day()),
        json!({
            "rule_file": "t.toml", "rules": [], "exceptions": [], "exceptions_in_force": 0, "expired": [],
            "exceptions_expired": 0,
        })
    );
}

#[test]
fn engine_takes_its_rule_file_name() {
    let text = r#"version = 1

[[rule]]
id = "lint"
engine = "swiftlint"
check = "force_cast"
fix = "Cast safely."
"#;
    assert_eq!(rules_json(Path::new("t.toml"), &parse(text), &all_available, day())["rules"][0]["engine"], json!("swiftlint"));
}

#[test]
fn each_rule_carries_whether_its_engine_is_available_and_why_not() {
    let text = r#"version = 1

[[rule]]
id = "py-lint"
engine = "ruff"
check = "E501"
fix = "Shorten the line."

[[rule]]
id = "no-force-cast"
engine = "swiftlint"
check = "force_cast"
fix = "Cast safely."
"#;
    let availability = |engine: Engine| match engine {
        Engine::Ruff => Availability::Available,
        Engine::SwiftLint => Availability::NotAvailable { why: "swiftlint not found on PATH".to_string() },
        other => panic!("availability asked for {other:?}"),
    };
    assert_eq!(
        rules_json(Path::new("t.toml"), &parse(text), &availability, day())["rules"],
        json!([
            {"id": "py-lint", "engine": "ruff", "kind": "quality", "severity": "error", "check": "E501",
             "paths": [], "exclude": [], "kits": [], "outside_kits": [], "available": true, "why": null},
            {"id": "no-force-cast", "engine": "swiftlint", "kind": "quality", "severity": "error",
             "check": "force_cast", "paths": [], "exclude": [], "kits": [], "outside_kits": [],
             "available": false, "why": "swiftlint not found on PATH"},
        ])
    );
}

#[test]
fn outside_kits_lists_the_rules_outside_kits_in_file_order() {
    let text = r#"version = 1

[kits.a]
paths = ["a/**"]

[kits.b]
paths = ["b/**"]

[[rule]]
id = "lint"
engine = "clippy"
check = "clippy::all"
outside_kits = ["a", "b"]
fix = "Do what clippy says."
"#;
    let value = rules_json(Path::new("t.toml"), &parse(text), &all_available, day());
    assert_eq!(value["rules"][0]["outside_kits"], json!(["a", "b"]));
    assert_eq!(value["rules"][0]["kits"], json!([]));
}

#[test]
fn an_exception_without_what_has_no_what_key_and_one_with_what_keeps_it() {
    let value = rules_json(Path::new("t.toml"), &parse(FULL), &all_available, day());
    assert_eq!(value["exceptions"][0]["what"], json!("unwrap"));
    let without = value["exceptions"][1].as_object().unwrap();
    assert!(without.get("what").is_none(), "{without:?}");
    assert_eq!(without.len(), 4, "{without:?}");
}

#[test]
fn an_expired_exception_moves_to_expired_with_its_expires_date() {
    let text = r#"version = 1

[[rule]]
id = "lint-all"
engine = "clippy"
check = "clippy::all"
fix = "Do what clippy says."

[[exception]]
rule = "lint-all"
path = "src/old.rs"
what = "dead_code"
reason = "Old."
approved = "bob 2026-01-02"
expires = "2026-01-31"

[[exception]]
rule = "lint-all"
path = "src/new.rs"
reason = "New."
approved = "bob 2026-11-17"
expires = "2099-12-31"
"#;
    let value = rules_json(Path::new("t.toml"), &parse(text), &all_available, day());
    assert_eq!(
        (&value["exceptions"], &value["exceptions_in_force"], &value["expired"], &value["exceptions_expired"]),
        (
            &json!([{"rule": "lint-all", "path": "src/new.rs", "reason": "New.", "approved": "bob 2026-11-17"}]),
            &json!(1),
            &json!([{"rule": "lint-all", "path": "src/old.rs", "what": "dead_code", "reason": "Old.",
                     "approved": "bob 2026-01-02", "expires": "2026-01-31"}]),
            &json!(1),
        )
    );
}
