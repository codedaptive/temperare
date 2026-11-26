//! ZUT for findings_from.rs: every field filled from the rule, the owner and the hit; hits cut by the rule's scope and
//! by `only` (a file or a folder entry); hit order kept. The rule file is built by `RuleFile::parse` on in-memory text.

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

const SRC_TEXT: &str = r#"version = 1
owner = "repo"
[[rule]]
id = "no-none-compare"
engine = "ruff"
kind = "security"
check = "E711"
paths = ["src/**", "srcx/**"]
severity = "warning"
fix = "Compare with `is None`."
"#;

fn hit(path: &str, line: u32, col: Option<u32>, what: &str) -> Hit {
    Hit { path: RelPath::new(path), line: Some(line), col, what: what.to_string() }
}

fn finding(owner: &str, path: &str, line: u32, col: Option<u32>, what: &str) -> Finding {
    let mut finding = Finding::new(
        "no-none-compare",
        (Engine::Ruff, Kind::Security, Severity::Warning),
        RelPath::new(path),
        line,
        col,
        what,
        "Compare with `is None`.",
    );
    finding.owner = Some(owner.to_string());
    finding
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
fn a_folder_entry_keeps_direct_and_nested_hits() {
    let file = RuleFile::parse(SRC_TEXT, Path::new("/repo")).unwrap();
    let hits = vec![hit("src/a.rs", 1, None, "direct"), hit("src/x/b.rs", 2, Some(3), "nested")];
    assert_eq!(
        findings_from(&file, &file.rules[0], hits, &[RelPath::new("src")]),
        vec![finding("repo", "src/a.rs", 1, None, "direct"), finding("repo", "src/x/b.rs", 2, Some(3), "nested")]
    );
}

#[test]
fn a_folder_entry_does_not_keep_a_prefix_sibling() {
    let file = RuleFile::parse(SRC_TEXT, Path::new("/repo")).unwrap();
    let hits = vec![hit("src/a.rs", 1, None, "inside"), hit("srcx/a.rs", 2, None, "sibling")];
    assert_eq!(
        findings_from(&file, &file.rules[0], hits, &[RelPath::new("src")]),
        vec![finding("repo", "src/a.rs", 1, None, "inside")]
    );
}

#[test]
fn a_folder_entry_still_respects_rule_exclusions() {
    assert_eq!(
        run(&["py"]),
        vec![
            finding("py-zone", "py/a.py", 3, Some(7), "comparison to None (E711)"),
            finding("py-zone", "py/b.py", 12, Some(4), "second in py"),
        ]
    );
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
    let quality_error = (Engine::Clippy, Kind::Quality, Severity::Error);
    let want = Finding::new("r", quality_error, RelPath::new("src/lib.rs"), 1, Some(2), "m", "f");
    assert_eq!(got, vec![want]);
}

#[test]
fn a_hit_on_line_zero_gives_a_finding_with_no_line() {
    let file = RuleFile::parse(TEXT, Path::new("/repo")).unwrap();
    let got = findings_from(&file, &file.rules[0], vec![hit("py/a.py", 0, None, "no line")], &[]);
    assert_eq!(got.len(), 1);
    assert_eq!(got[0].line(), None);
}

#[test]
fn a_hit_with_no_line_gives_a_finding_with_no_line() {
    let file = RuleFile::parse(TEXT, Path::new("/repo")).unwrap();
    let no_line = Hit { path: RelPath::new("py/a.py"), line: None, col: None, what: "no line".to_string() };
    let got = findings_from(&file, &file.rules[0], vec![no_line], &[]);
    assert_eq!(got.len(), 1);
    assert_eq!(got[0].line(), None);
}
