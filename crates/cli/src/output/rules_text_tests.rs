//! ZUT for rules_text.rs: the listing's exact lines, from rule files parsed from literal TOML (no disk) and engine
//! availability from closures (never the real PATH).

use super::rules_text;
use engines::Availability;
use rules::Engine;
use std::path::Path;

fn all_available(_: Engine) -> Availability {
    Availability::Available
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
paths = ["src/**", "lib/**"]
exclude = ["**/tests/**", "**/gen/**"]
kits = ["core"]
severity = "warning"
fix = "Return a typed error."

[[exception]]
rule = "no-unwrap"
path = "src/a.rs"
what = "unwrap"
reason = "Legacy."
approved = "bob 2026-11-17"

[[exception]]
rule = "lint-all"
path = "src/b.rs"
reason = "Generated."
approved = "bob 2026-11-16"
"#;

#[test]
fn rules_and_exceptions_in_file_order_with_scope_and_optional_what() {
    let expected = "rule file: repo/temperare.toml\n\
                    rules (2):\n  \
                    lint-all  clippy  quality  error  paths: all files  available\n  \
                    no-unwrap  clippy  security  warning  paths: src/**, lib/**  exclude: **/tests/**, **/gen/**  kits: core  available\n\
                    exceptions in force (2):\n  \
                    no-unwrap  src/a.rs  what: unwrap  approved: bob 2026-11-17  reason: Legacy.\n  \
                    lint-all  src/b.rs  approved: bob 2026-11-16  reason: Generated.\n";
    assert_eq!(rules_text(Path::new("repo/temperare.toml"), &parse(FULL), &all_available), expected);
}

#[test]
fn an_empty_file_prints_both_headers_with_zero() {
    assert_eq!(
        rules_text(Path::new("t.toml"), &parse("version = 1\n"), &all_available),
        "rule file: t.toml\nrules (0):\nexceptions in force (0):\n"
    );
}

#[test]
fn engine_names_are_the_rule_file_names() {
    let text = r#"version = 1

[[rule]]
id = "lint"
engine = "ruff"
check = "E501"
paths = ["**/*.py"]
fix = "Shorten the line."
"#;
    assert_eq!(
        rules_text(Path::new("t.toml"), &parse(text), &all_available),
        "rule file: t.toml\nrules (1):\n  lint  ruff  quality  error  paths: **/*.py  available\nexceptions in force (0):\n"
    );
}

#[test]
fn each_rule_ends_with_its_engines_availability_after_the_scope() {
    let text = r#"version = 1

[kits.app]
paths = ["App/**"]

[[rule]]
id = "py-lint"
engine = "ruff"
check = "E501"
paths = ["**/*.py"]
exclude = ["gen/**"]
fix = "Shorten the line."

[[rule]]
id = "no-force-cast"
engine = "swiftlint"
check = "force_cast"
kits = ["app"]
fix = "Cast safely."
"#;
    let availability = |engine: Engine| match engine {
        Engine::Ruff => Availability::Available,
        Engine::SwiftLint => Availability::NotAvailable { why: "swiftlint not found on PATH".to_string() },
        other => panic!("availability asked for {other:?}"),
    };
    assert_eq!(
        rules_text(Path::new("t.toml"), &parse(text), &availability),
        "rule file: t.toml\n\
         rules (2):\n  \
         py-lint  ruff  quality  error  paths: **/*.py  exclude: gen/**  available\n  \
         no-force-cast  swiftlint  quality  error  paths: all files  kits: app  not available: swiftlint not found on PATH\n\
         exceptions in force (0):\n"
    );
}
