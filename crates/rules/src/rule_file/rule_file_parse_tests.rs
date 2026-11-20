//! Unit tests for rule_file_parse.rs: the numbered checks one case each, then wrong types, `kind`, `owner`, syntax errors and problem order, on
//! in-memory text, with the expected line and column written out.

use crate::{Engine, Kind, Problem, RelPath, RuleFile, Severity};
use std::path::Path;

/// A valid rule, to which a case adds lines.
const RULE: &str = "[[rule]]\nid = \"r\"\nengine = \"clippy\"\ncheck = \"clippy::all\"\nfix = \"f\"\n";

fn problems(text: &str) -> Vec<Problem> {
    RuleFile::parse(text, Path::new(".")).expect_err("the text has a problem")
}

/// The one problem of `text`, at `line`:`col`, whose `what` contains `names`.
fn only(text: &str, line: usize, col: usize, names: &str) {
    let found = problems(text);
    assert_eq!(found.len(), 1, "{found:?}");
    assert_eq!((found[0].line, found[0].col), (line, col), "{found:?}");
    assert!(found[0].what.contains(names), "{:?} should name {names}", found[0].what);
    assert!(!found[0].fix.is_empty());
}

#[test]
fn check_1_unknown_key_at_top_level_in_a_kit_in_a_rule_and_in_an_exception() {
    only("version = 1\ncolour = \"blue\"\n", 2, 1, "colour");
    only("version = 1\n[kits.core]\npaths = [\"a/**\"]\ncolours = 1\n", 4, 1, "colours");
    only(&format!("version = 1\n{RULE}  sevrity = \"error\"\n"), 7, 3, "sevrity");
    let exception = "[[exception]]\nrule = \"r\"\npath = \"a.rs\"\nreason = \"x\"\napproved = \"bob\"\nline = 3\n";
    only(&format!("version = 1\n{RULE}{exception}"), 12, 1, "line");
}

#[test]
fn check_2_unknown_engine_at_the_value() {
    only("version = 1\n[[rule]]\nid = \"r\"\nengine = \"eslint\"\ncheck = \"c\"\nfix = \"f\"\n", 4, 10, "eslint");
}

#[test]
fn check_3_duplicate_id_at_the_second_value() {
    only(&format!("version = 1\n{RULE}{RULE}"), 8, 6, "\"r\"");
}

#[test]
fn check_4_malformed_id_at_the_value() {
    only("version = 1\n[[rule]]\nid = \"No_Good\"\nengine = \"ruff\"\ncheck = \"E1\"\nfix = \"f\"\n", 3, 6, "No_Good");
    only("version = 1\n[[rule]]\nid = \"\"\nengine = \"ruff\"\ncheck = \"E1\"\nfix = \"f\"\n", 3, 6, "id");
}

#[test]
fn check_5_missing_fix_at_the_rule_header() {
    only("version = 1\n\n[[rule]]\nid = \"r\"\nengine = \"ruff\"\ncheck = \"E1\"\n", 3, 1, "`fix`");
}

#[test]
fn check_6_missing_check_at_the_rule_header() {
    only("version = 1\n  [[rule]]\nid = \"r\"\nengine = \"ruff\"\nfix = \"f\"\n", 2, 3, "`check`");
}

#[test]
fn check_7_bad_glob_in_paths_exclude_and_a_kits_paths_at_that_string() {
    only(&format!("version = 1\n{RULE}paths = [\"ok/**\", \"src/[a.rs\"]\n"), 7, 19, "src/[a.rs");
    only(&format!("version = 1\n{RULE}exclude = [\"gen/[x\"]\n"), 7, 12, "gen/[x");
    only("version = 1\n[kits.core]\npaths = [\"core/[\"]\n", 3, 10, "core/[");
}

#[test]
fn check_8_kit_named_but_not_defined_at_that_string() {
    let text = format!("version = 1\n[kits.core]\npaths = [\"c/**\"]\n{RULE}kits = [\"core\", \"web\"]\n");
    only(&text, 9, 17, "\"web\"");
}

#[test]
fn check_9_exception_for_an_unknown_rule_at_the_value() {
    let exception = "[[exception]]\nrule = \"ghost\"\npath = \"a.rs\"\nreason = \"x\"\napproved = \"bob\"\n";
    only(&format!("version = 1\n{RULE}{exception}"), 8, 8, "ghost");
}

#[test]
fn check_10_exception_missing_reason_at_its_header() {
    let exception = "[[exception]]\nrule = \"r\"\npath = \"a.rs\"\napproved = \"bob\"\n";
    only(&format!("version = 1\n{RULE}{exception}"), 7, 1, "`reason`");
}

#[test]
fn check_11_exception_missing_approved_at_its_header() {
    let exception = "[[exception]]\nrule = \"r\"\npath = \"a.rs\"\nreason = \"x\"\n";
    only(&format!("version = 1\n{RULE}{exception}"), 7, 1, "`approved`");
}

#[test]
fn check_12_version_absent_at_1_1() {
    only(&format!("\n\n{RULE}"), 1, 1, "version");
}

#[test]
fn check_13_version_not_the_integer_1_at_the_value() {
    only("version = 2\n", 1, 11, "version");
    only("version = \"1\"\n", 1, 11, "version");
}

#[test]
fn a_value_of_the_wrong_type_is_a_problem_at_the_value() {
    only(&format!("version = 1\n{RULE}paths = \"src/**\"\n"), 7, 9, "paths");
    only("version = 1\nkits = 3\n", 2, 8, "kits");
}

#[test]
fn a_rule_missing_id_or_engine_is_a_problem_at_its_header() {
    only("version = 1\n[[rule]]\nengine = \"ruff\"\ncheck = \"E1\"\nfix = \"f\"\n", 2, 1, "`id`");
    only("version = 1\n[[rule]]\nid = \"r\"\ncheck = \"E1\"\nfix = \"f\"\n", 2, 1, "`engine`");
}

#[test]
fn an_exception_missing_rule_or_path_is_a_problem_at_its_header() {
    only(&format!("version = 1\n{RULE}[[exception]]\npath = \"a.rs\"\nreason = \"x\"\napproved = \"b\"\n"), 7, 1, "`rule`");
    only(&format!("version = 1\n{RULE}[[exception]]\nrule = \"r\"\nreason = \"x\"\napproved = \"b\"\n"), 7, 1, "`path`");
}

#[test]
fn an_unknown_severity_is_a_problem_at_the_value() {
    only(&format!("version = 1\n{RULE}severity = \"fatal\"\n"), 7, 12, "fatal");
}

#[test]
fn an_unknown_kind_is_a_problem_at_the_value() {
    only(&format!("version = 1\n{RULE}kind = \"style\"\n"), 7, 8, "style");
    let found = problems(&format!("version = 1\n{RULE}kind = \"style\"\n"));
    assert!(found[0].fix.contains("quality") && found[0].fix.contains("security"), "{}", found[0].fix);
}

#[test]
fn a_kind_of_the_wrong_type_is_a_problem_at_the_value() {
    only(&format!("version = 1\n{RULE}kind = [\"security\"]\n"), 7, 8, "kind");
}

#[test]
fn a_rule_kind_is_read_and_defaults_to_quality() {
    let text = format!("version = 1\nowner = \"core\"\n{RULE}{SECURITY_RULE}");
    let file = RuleFile::parse(&text, Path::new(".")).expect("valid");
    let kinds: Vec<Kind> = file.rules.iter().map(|rule| rule.kind).collect();
    assert_eq!(kinds, vec![Kind::Quality, Kind::Security]);
}

/// A valid security rule; its `kind` value is on its fourth line, at column 8.
const SECURITY_RULE: &str = "[[rule]]\nid = \"s\"\nengine = \"tessera\"\nkind = \"security\"\ncheck = \"c\"\nfix = \"f\"\n";

#[test]
fn a_security_rule_without_a_top_level_owner_is_a_problem_at_its_kind() {
    only(&format!("version = 1\n{RULE}{SECURITY_RULE}"), 10, 8, "owner");
    let found = problems(&format!("version = 1\n{RULE}{SECURITY_RULE}"));
    assert!(found[0].fix.contains("owner = "), "{}", found[0].fix);
}

#[test]
fn a_security_rule_with_a_blank_top_level_owner_is_a_problem_at_its_kind() {
    only(&format!("version = 1\nowner = \"  \"\n{SECURITY_RULE}"), 6, 8, "owner");
}

#[test]
fn a_security_rule_with_a_top_level_owner_is_no_problem() {
    let file = RuleFile::parse(&format!("version = 1\nowner = \"core\"\n{SECURITY_RULE}"), Path::new("."))
        .expect("valid");
    assert_eq!(file.owner.as_deref(), Some("core"));
}

#[test]
fn a_top_level_owner_that_is_not_a_string_with_a_security_rule_is_two_problems() {
    let found: Vec<(usize, usize)> = problems(&format!("version = 1\nowner = 3\n{SECURITY_RULE}"))
        .iter()
        .map(|p| (p.line, p.col))
        .collect();
    assert_eq!(found, vec![(2, 9), (6, 8)]);
}

#[test]
fn owners_are_read_as_written_and_a_blank_one_is_none() {
    let text = "version = 1\nowner = \" repo \"\n[kits.core]\npaths = [\"c/**\"]\nowner = \" core \"\n";
    let file = RuleFile::parse(text, Path::new(".")).expect("an owner is an allowed key at the top and in a kit");
    assert_eq!(file.owner.as_deref(), Some(" repo "));
    assert_eq!(file.kits[0].owner.as_deref(), Some(" core "));
    let blank = RuleFile::parse("version = 1\nowner = \"\"\n", Path::new(".")).expect("valid");
    assert_eq!(blank.owner, None);
}

#[test]
fn a_toml_syntax_error_is_one_problem_at_its_position() {
    let found = problems("version = 1\nid = = 3\n");
    assert_eq!(found.len(), 1, "{found:?}");
    assert_eq!((found[0].line, found[0].col), (2, 6), "{found:?}");
    assert!(found[0].what.contains("syntax"), "{}", found[0].what);
}

#[test]
fn every_problem_in_a_file_comes_back_in_line_and_column_order() {
    let text = "version = 3\nzeta = 1\nalpha = 2\n[[rule]]\nid = \"Bad\"\nengine = \"nope\"\n\
                [[exception]]\nrule = \"ghost\"\npath = \"a.rs\"\n";
    let found: Vec<(usize, usize)> = problems(text).iter().map(|p| (p.line, p.col)).collect();
    let expected = vec![(1, 11), (2, 1), (3, 1), (4, 1), (4, 1), (5, 6), (6, 10), (7, 1), (7, 1), (8, 8)];
    assert_eq!(found, expected);
}

#[test]
fn a_file_with_only_the_version_is_valid_and_empty() {
    let file = RuleFile::parse("version = 1\n", Path::new(".")).expect("valid");
    assert!(file.kits.is_empty() && file.rules.is_empty() && file.exceptions.is_empty());
}

/// A full example rule file, with a comment on each part.
const SPEC_EXAMPLE: &str = r#"version = 1

[kits.core]                      # optional: a named set of paths
paths = ["Sources/Core/**", "crates/core/**"]

[[rule]]
id = "no-stored-bool"            # unique, [a-z0-9-]+
engine = "ast-grep"              # clippy | swiftlint | ruff | ast-grep | tessera
check = "rules/no-stored-bool.yml"   # what the engine is asked: a lint name, a code, a rule file
config = ".swiftlint.yml"        # optional: the engine's own configuration, pointed at, never copied
paths = ["**/*.swift", "**/*.rs"]   # include globs, relative to the rule file's folder; default all
exclude = ["**/generated/**"]    # optional
kits = ["core"]                  # optional: also limited to these kits' paths
severity = "error"               # error | warning
fix = "Store a bitmap field; expose the Bool as a computed accessor."   # required: a refusal names the fix
params = { entity_types = ["Order", "Invoice"] }   # optional: free table handed to the engine adapter

[[exception]]
rule = "no-stored-bool"
path = "Sources/Core/Legacy.swift"   # one file, exact
what = "isArchived"              # optional: matches only findings whose `what` contains this text
reason = "migration ships in 2.4"
approved = "Bob 2026-11-17"      # required
"#;

#[test]
fn the_spec_example_parses_to_its_fields() {
    let file = RuleFile::parse(SPEC_EXAMPLE, Path::new("repo")).expect("the spec's example is valid");
    assert_eq!(file.kits.len(), 1);
    assert_eq!(file.kits[0].name, "core");
    assert_eq!(file.kits[0].paths, vec!["Sources/Core/**".to_string(), "crates/core/**".to_string()]);
    assert_eq!(file.owner, None, "the example names no owner");
    assert_eq!(file.rules.len(), 1);
    let rule = &file.rules[0];
    assert_eq!(rule.id, "no-stored-bool");
    assert_eq!(rule.engine, Engine::AstGrep);
    assert_eq!(rule.kind, Kind::Quality, "the example gives no kind");
    assert_eq!(rule.check, "rules/no-stored-bool.yml");
    assert_eq!(rule.config.as_deref(), Some(".swiftlint.yml"));
    assert_eq!(rule.severity, Severity::Error);
    assert_eq!(rule.fix, "Store a bitmap field; expose the Bool as a computed accessor.");
    let entity_types = rule.params["entity_types"].as_array().expect("an array");
    let names: Vec<&str> = entity_types.iter().filter_map(|v| v.as_str()).collect();
    assert_eq!(names, vec!["Order", "Invoice"]);
    assert_eq!(rule.kits, vec!["core".to_string()]);
    assert!(rule.applies(&RelPath::new("Sources/Core/Model.swift")));
    assert!(rule.applies(&RelPath::new("crates/core/src/lib.rs")));
    assert!(!rule.applies(&RelPath::new("Sources/App/Model.swift")), "outside the kit");
    assert!(!rule.applies(&RelPath::new("crates/core/generated/x.rs")), "excluded");
    assert!(!rule.applies(&RelPath::new("Sources/Core/Model.py")), "not in paths");
    assert_eq!(file.exceptions.len(), 1);
    let exception = &file.exceptions[0];
    assert_eq!(exception.rule, "no-stored-bool");
    assert_eq!(exception.path, RelPath::new("Sources/Core/Legacy.swift"));
    assert_eq!(exception.what.as_deref(), Some("isArchived"));
    assert_eq!(exception.reason, "migration ships in 2.4");
    assert_eq!(exception.approved, "Bob 2026-11-17");
}
