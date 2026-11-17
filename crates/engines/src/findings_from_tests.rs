//! ZUT for findings_from.rs: every field filled from the rule, the owner and the hit; hits cut by the rule's scope and
//! by `only`; hit order kept. The rule file is built by `RuleFile::parse` on in-memory text.

use crate::findings_from::findings_from;
use crate::types::Hit;
use rules::{Engine, Finding, Kind, RelPath, RuleFile, Severity};
use std::path::Path;

/// `py/` is the py-zone's kit; everything else is the top-level owner's. The rule covers `py/` and `tools/` but not
/// `py/gen/`, and sets a kind and severity other than the defaults.
const TEXT: &str = r#"version = 1
owner = "repo"
[kits.py]
paths = ["py/**"]
owner = "py-zone"
[[rule]]
id = "no-none-compare"
engine = "ruff"
kind = "security"
check = "E711"
paths = ["py/**", "tools/**"]
exclude = ["py/gen/**"]
severity = "warning"
fix = "Compare with `is None`."
"#;

fn hit(path: &str, line: u32, col: Option<u32>, what: &str) -> Hit {
    Hit { path: RelPath::new(path), line, col, what: what.to_string() }
}

fn finding(owner: &str, path: &str, line: u32, col: Option<u32>, what: &str) -> Finding {
    Finding {
        rule: "no-none-compare".to_string(),
        engine: Engine::Ruff,
        kind: Kind::Security,
        severity: Severity::Warning,
        owner: Some(owner.to_string()),
        path: RelPath::new(path),
        line,
        col,
        what: what.to_string(),
        fix: "Compare with `is None`.".to_string(),
    }
}

fn hits() -> Vec<Hit> {
    vec![
        hit("tools/t.py", 9, None, "comparison to None"),
        hit("py/gen/x.py", 1, Some(1), "generated"),
        hit("py/a.py", 3, Some(7), "comparison to None (E711)"),
        hit("docs/d.py", 2, Some(2), "outside the rule's paths"),
        hit("py/b.py", 12, Some(4), "second in py"),
    ]
}

fn run(only: &[&str]) -> Vec<Finding> {
    let file = RuleFile::parse(TEXT, Path::new("/repo")).unwrap();
    let only: Vec<RelPath> = only.iter().map(|p| RelPath::new(p)).collect();
    findings_from(&file, &file.rules[0], hits(), &only)
}

#[test]
fn with_no_only_every_hit_the_rule_applies_to_is_a_filled_finding_in_hit_order() {
    assert_eq!(
        run(&[]),
        vec![
            finding("repo", "tools/t.py", 9, None, "comparison to None"),
            finding("py-zone", "py/a.py", 3, Some(7), "comparison to None (E711)"),
            finding("py-zone", "py/b.py", 12, Some(4), "second in py"),
        ]
    );
}

#[test]
fn only_keeps_just_the_listed_paths() {
    assert_eq!(run(&["py/b.py"]), vec![finding("py-zone", "py/b.py", 12, Some(4), "second in py")]);
}

#[test]
fn a_path_in_only_that_the_rule_does_not_apply_to_is_still_dropped() {
    assert_eq!(run(&["py/gen/x.py", "docs/d.py"]), Vec::<Finding>::new());
}

#[test]
fn a_file_with_no_owner_anywhere_has_none() {
    let text = "version = 1\n[[rule]]\nid = \"r\"\nengine = \"clippy\"\ncheck = \"clippy::bool_comparison\"\nfix = \"f\"\n";
    let file = RuleFile::parse(text, Path::new("/repo")).unwrap();
    let got = findings_from(&file, &file.rules[0], vec![hit("src/lib.rs", 1, Some(2), "m")], &[]);
    let want = Finding {
        rule: "r".to_string(),
        engine: Engine::Clippy,
        kind: Kind::Quality,
        severity: Severity::Error,
        owner: None,
        path: RelPath::new("src/lib.rs"),
        line: 1,
        col: Some(2),
        what: "m".to_string(),
        fix: "f".to_string(),
    };
    assert_eq!(got, vec![want]);
}
