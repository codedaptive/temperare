//! Unit tests for rule_file_parse.rs: one case per check of the rule file format and the `kind` and `owner` checks, on
//! in-memory text, with the expected line and column written out; then the `baseline` and `[[override]]` keys, on
//! baseline sets written here and passed to parse_with_sets.

use super::baseline_set::BaselineSet;
use super::rule_file_parse::parse_with_sets;
use crate::{Date, Engine, Kind, Problem, RelPath, Rule, RuleFile, Severity};
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
    let exception = "[[exception]]\nrule = \"r\"\npath = \"a.rs\"\nreason = \"x\"\napproved = \"bob\"\nline = 3\n\
                    expires = \"2099-12-31\"\n";
    only(&format!("version = 1\n{RULE}{exception}"), 12, 1, "line");
}

#[test]
fn check_2_unknown_engine_at_the_value() {
    only("version = 1\n[[rule]]\nid = \"r\"\nengine = \"nolint\"\ncheck = \"c\"\nfix = \"f\"\n", 4, 10, "nolint");
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
    let exception = "[[exception]]\nrule = \"ghost\"\npath = \"a.rs\"\nreason = \"x\"\napproved = \"bob\"\n\
                    expires = \"2099-12-31\"\n";
    only(&format!("version = 1\n{RULE}{exception}"), 8, 8, "ghost");
}

#[test]
fn check_10_exception_missing_reason_at_its_header() {
    let exception = "[[exception]]\nrule = \"r\"\npath = \"a.rs\"\napproved = \"bob\"\nexpires = \"2099-12-31\"\n";
    only(&format!("version = 1\n{RULE}{exception}"), 7, 1, "`reason`");
}

#[test]
fn check_11_exception_missing_approved_at_its_header() {
    let exception = "[[exception]]\nrule = \"r\"\npath = \"a.rs\"\nreason = \"x\"\nexpires = \"2099-12-31\"\n";
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
    only(&format!("version = 1\n{RULE}[[exception]]\npath = \"a.rs\"\nreason = \"x\"\napproved = \"b\"\n\
                    expires = \"2099-12-31\"\n"), 7, 1, "`rule`");
    only(&format!("version = 1\n{RULE}[[exception]]\nrule = \"r\"\nreason = \"x\"\napproved = \"b\"\n\
                    expires = \"2099-12-31\"\n"), 7, 1, "`path`");
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
                [[exception]]\nrule = \"ghost\"\nexpires = \"2099-12-31\"\npath = \"a.rs\"\n";
    let found: Vec<(usize, usize)> = problems(text).iter().map(|p| (p.line, p.col)).collect();
    let expected = vec![(1, 11), (2, 1), (3, 1), (4, 1), (4, 1), (5, 6), (6, 10), (7, 1), (7, 1), (8, 8)];
    assert_eq!(found, expected);
}

#[test]
fn a_file_with_only_the_version_is_valid_and_empty() {
    let file = RuleFile::parse("version = 1\n", Path::new(".")).expect("valid");
    assert!(file.kits.is_empty() && file.rules.is_empty() && file.exceptions.is_empty());
}

/// The SPEC's example rule file (WO-rules-1), with the `expires` every exception now needs.
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
severity = "error"               # error | warning | watch
fix = "Store a bitmap field; expose the Bool as a computed accessor."   # required: a refusal names the fix
params = { entity_types = ["Order", "Invoice"] }   # optional: free table handed to the engine adapter

[[exception]]
rule = "no-stored-bool"
path = "Sources/Core/Legacy.swift"   # one file, exact
what = "isArchived"              # optional: matches only findings whose `what` contains this text
reason = "migration ships in 2.4"
approved = "Bob 2026-11-17"      # required
expires = "2099-12-31"           # required: the last day it holds, in UTC
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
    assert_eq!(exception.expires, Date::new(2099, 12, 31));
}

/// The baseline sets the cases below take, in place of the sets built into `cctao`.
const SETS: &[BaselineSet] = &[
    BaselineSet {
        name: "standard",
        version: 1,
        ready: true,
        text: "version = 1\nretired = [\"tm-s-old\"]\n[[rule]]\nid = \"tm-s1\"\nengine = \"clippy\"\ncheck = \"s1\"\n\
               fix = \"f\"\npaths = [\"**/*.rs\"]\nparams = { limit = 10, keep = \"k\" }\n\
               failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n",
        yaml: &[],
        cases: &[("cases/tm-s1/failing/a.rs", "x\n"), ("cases/tm-s1/passing/a.rs", "x\n")],
    },
    BaselineSet { name: "standard", version: 2, ready: true, text: "version = 1\n", yaml: &[], cases: &[] },
    BaselineSet {
        name: "house",
        version: 1,
        ready: true,
        text: "version = 1\nretired = [\"tm-h-old\"]\n[[rule]]\nid = \"tm-h1\"\nengine = \"ruff\"\ncheck = \"h1\"\nfix = \"f\"\n\
               failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n\
               [[rule]]\nid = \"tm-h2\"\nengine = \"ast-grep\"\ncheck = \"h2\"\nfix = \"f\"\nrequired_params = [\"entity_types\"]\n\
               failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n",
        yaml: &[("h2", "rule: h2\n")],
        cases: &[
            ("cases/tm-h1/failing/a.rs", "x\n"),
            ("cases/tm-h1/passing/a.rs", "x\n"),
            ("cases/tm-h2/failing/a.rs", "x\n"),
            ("cases/tm-h2/passing/a.rs", "x\n"),
        ],
    },
    BaselineSet { name: "broken", version: 1, ready: true, text: "version = 1\ncolour = 1\n", yaml: &[], cases: &[] },
    BaselineSet {
        name: "dup",
        version: 1,
        ready: true,
        text: "version = 1\n[[rule]]\nid = \"tm-s1\"\nengine = \"ruff\"\ncheck = \"other\"\nfix = \"f\"\n\
               failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n",
        yaml: &[],
        cases: &[("cases/tm-s1/failing/a.rs", "x\n"), ("cases/tm-s1/passing/a.rs", "x\n")],
    },
];

/// Takes standard@1 then house@1; an override's `rule` string is on line 4, a key after it on line 5.
const BOTH: &str = "version = 1\nbaseline = [\"standard@1\", \"house@1\"]\n";
const TAKES_STANDARD: &str = "version = 1\nbaseline = [\"standard@1\"]\n";
const H2_REASON: &str =
    "required params not given: entity_types; give them in the params of an [[override]] for \"tm-h2\"";

fn parse(text: &str) -> Result<RuleFile, Vec<Problem>> {
    parse_with_sets(text, Path::new("."), SETS)
}

fn valid(text: &str) -> RuleFile {
    parse(text).unwrap_or_else(|p| panic!("valid, but: {p:?}"))
}

fn ids(rules: &[Rule]) -> Vec<&str> {
    rules.iter().map(|rule| rule.id.as_str()).collect()
}

/// `text`'s problems are exactly one, at `line`:`col`, with this what and fix.
fn exactly(text: &str, line: usize, col: usize, what: &str, fix: &str) {
    let expected = Problem { line, col, what: what.to_string(), fix: fix.to_string() };
    assert_eq!(parse(text).expect_err("the text has a problem"), vec![expected]);
}

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|item| item.to_string()).collect()
}

#[test]
fn baseline_absent_reads_as_before_with_the_new_fields_empty() {
    let file = valid(&format!("version = 1\n{RULE}"));
    assert_eq!(ids(&file.rules), vec!["r"]);
    assert_eq!(file.baseline, Vec::<String>::new());
    assert_eq!(file.retired, Vec::<String>::new());
    assert_eq!(ids(&file.not_applicable_rules), Vec::<&str>::new());
}

#[test]
fn baseline_sets_rules_come_first_then_the_files_own() {
    let file = valid(&format!("{BOTH}{RULE}"));
    assert_eq!(ids(&file.rules), vec!["tm-s1", "tm-h1", "r"]);
    assert_eq!(file.baseline, strings(&["standard@1", "house@1"]));
    assert_eq!(file.retired, strings(&["tm-s-old", "tm-h-old"]));
    assert_eq!(ids(&file.not_applicable_rules), vec!["tm-h2"]);
    assert_eq!(file.not_applicable_rules[0].not_applicable.as_deref(), Some(H2_REASON));
    assert_eq!(file.rules[0].not_applicable, None);
}

#[test]
fn baseline_override_severity_changes_only_its_rule() {
    let file = valid(&format!("{BOTH}[[override]]\nrule = \"tm-s1\"\nseverity = \"warning\"\n"));
    assert_eq!(file.rules[0].severity, Severity::Warning);
    assert_eq!(file.rules[1].severity, Severity::Error);
}

#[test]
fn baseline_override_paths_and_exclude_rescope_the_rule() {
    let file = valid(&format!("{TAKES_STANDARD}[[override]]\nrule = \"tm-s1\"\npaths = [\"src/**\"]\nexclude = [\"src/gen/**\"]\n"));
    let rule = &file.rules[0];
    assert!(rule.applies(&RelPath::new("src/a.py")), "in the override's paths");
    assert!(!rule.applies(&RelPath::new("lib/a.rs")), "in the set's paths only");
    assert!(!rule.applies(&RelPath::new("src/gen/a.rs")), "excluded");
}

#[test]
fn baseline_override_params_merge_into_the_sets() {
    let file = valid(&format!("{TAKES_STANDARD}[[override]]\nrule = \"tm-s1\"\nparams = {{ limit = 20, extra = true }}\n"));
    let expected: toml::Table = toml::from_str("limit = 20\nkeep = \"k\"\nextra = true\n").expect("table");
    assert_eq!(file.rules[0].params, expected);
}

#[test]
fn baseline_override_off_removes_the_rule_and_an_exception_may_still_name_it() {
    let exception = "[[exception]]\nrule = \"tm-s1\"\npath = \"a.rs\"\nreason = \"x\"\napproved = \"bob\"\n\
                    expires = \"2099-12-31\"\n";
    let file = valid(&format!("{BOTH}[[override]]\nrule = \"tm-s1\"\noff = true\n{exception}"));
    assert_eq!(ids(&file.rules), vec!["tm-h1"]);
    assert_eq!(ids(&file.not_applicable_rules), vec!["tm-h2"]);
    assert_eq!(file.exceptions.iter().map(|e| e.rule.as_str()).collect::<Vec<_>>(), vec!["tm-s1"]);
}

#[test]
fn baseline_required_param_given_by_an_override_makes_the_rule_apply() {
    let file = valid(&format!("{BOTH}[[override]]\nrule = \"tm-h2\"\nparams = {{ entity_types = [\"Order\"] }}\n"));
    assert_eq!(ids(&file.rules), vec!["tm-s1", "tm-h1", "tm-h2"]);
    assert_eq!(file.rules[2].not_applicable, None);
    assert_eq!(ids(&file.not_applicable_rules), Vec::<&str>::new());
}

#[test]
fn baseline_exception_naming_a_baseline_rule_is_accepted() {
    let exception = "[[exception]]\nrule = \"tm-h1\"\npath = \"a.rs\"\nreason = \"x\"\napproved = \"bob\"\n\
                    expires = \"2099-12-31\"\n";
    let file = valid(&format!("{BOTH}{exception}"));
    assert_eq!(file.exceptions.iter().map(|e| e.rule.as_str()).collect::<Vec<_>>(), vec!["tm-h1"]);
}

#[test]
fn baseline_problem_unknown_set() {
    let fix = "Use one of: broken@1, dup@1, house@1, standard@1, standard@2.";
    exactly("version = 1\nbaseline = [\"nosuch@1\"]\n", 2, 13, "unknown baseline set \"nosuch\"", fix);
}

#[test]
fn baseline_problem_unknown_version() {
    let what = "unknown version 9 of baseline set \"standard\"";
    exactly("version = 1\nbaseline = [\"standard@9\"]\n", 2, 13, what, "Use one of: standard@1, standard@2.");
}

#[test]
fn baseline_problem_malformed_entry() {
    let fix = "Write the entry as a set name, '@' and a whole-number version, e.g. \"standard@1\".";
    exactly("version = 1\nbaseline = [\"standard@one\"]\n", 2, 13, "malformed baseline entry \"standard@one\"", fix);
}

#[test]
fn baseline_problem_set_taken_twice() {
    let text = "version = 1\nbaseline = [\"standard@1\", \"standard@2\"]\n";
    exactly(text, 2, 27, "baseline set \"standard\" is taken twice", "Take each set once, at one version.");
}

#[test]
fn baseline_problem_own_id_with_the_baseline_prefix() {
    let text = format!("{TAKES_STANDARD}[[rule]]\nid = \"tm-mine\"\nengine = \"clippy\"\ncheck = \"c\"\nfix = \"f\"\n");
    let what = "id \"tm-mine\" starts with \"tm-\", which is kept for baseline rules";
    let fix = "Give the rule an id that does not start with \"tm-\", or change the baseline rule with an [[override]].";
    exactly(&text, 4, 6, what, fix);
}

const UNKNOWN_OVERRIDE_FIX: &str =
    "Name the id of a rule in a set this file takes in baseline, or remove the [[override]].";

#[test]
fn baseline_problem_override_of_an_id_not_in_the_taken_sets() {
    let what = "override of unknown baseline rule \"tm-h1\": no taken set holds it";
    exactly(&format!("{TAKES_STANDARD}[[override]]\nrule = \"tm-h1\"\n"), 4, 8, what, UNKNOWN_OVERRIDE_FIX);
}

#[test]
fn baseline_problem_override_with_no_baseline_key() {
    let what = "override of unknown baseline rule \"tm-s1\": no taken set holds it";
    exactly("version = 1\n[[override]]\nrule = \"tm-s1\"\n", 3, 8, what, UNKNOWN_OVERRIDE_FIX);
}

#[test]
fn baseline_problem_override_with_an_unknown_key() {
    let fix = "Remove it, or use one of: rule, severity, off, paths, exclude, params.";
    let text = format!("{TAKES_STANDARD}[[override]]\nrule = \"tm-s1\"\nlevel = 2\n");
    exactly(&text, 5, 1, "unknown key `level` in [[override]]", fix);
}

#[test]
fn baseline_problem_override_with_a_wrong_type() {
    let text = format!("{TAKES_STANDARD}[[override]]\nrule = \"tm-s1\"\npaths = \"x\"\n");
    let what = "`paths` must be an array of strings, not string";
    exactly(&text, 5, 9, what, "Write `paths` as an array of strings.");
}

#[test]
fn baseline_problem_override_off_not_a_boolean() {
    let text = format!("{TAKES_STANDARD}[[override]]\nrule = \"tm-s1\"\noff = \"yes\"\n");
    exactly(&text, 5, 7, "`off` must be a boolean, not string", "Write `off` as a boolean.");
}

#[test]
fn baseline_problem_a_set_files_own_problem() {
    let what = "baseline set broken@1 has a problem at 2:1: unknown key `colour` in the top level";
    let fix = "This is a fault in cctao's embedded set; report it with this message.";
    exactly("version = 1\nbaseline = [\"broken@1\"]\n", 2, 13, what, fix);
}

#[test]
fn baseline_problem_the_same_id_in_two_sets() {
    let text = "version = 1\nbaseline = [\"standard@1\", \"dup@1\"]\n";
    exactly(text, 2, 27, "rule \"tm-s1\" is in both standard@1 and dup@1", "Take only one of the two sets.");
}

/// Baseline sets holding a security rule: `guard` (`tm-g1`), and `guardneeds` (`tm-n1`, which needs `types`); and
/// `plain`, a quality rule only.
const SECURITY_SETS: &[BaselineSet] = &[
    BaselineSet {
        name: "plain",
        version: 1,
        ready: true,
        text: "version = 1\n[[rule]]\nid = \"tm-p1\"\nengine = \"ruff\"\ncheck = \"p1\"\nfix = \"f\"\n\
               failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n",
        yaml: &[],
        cases: &[("cases/tm-p1/failing/a.rs", "x\n"), ("cases/tm-p1/passing/a.rs", "x\n")],
    },
    BaselineSet {
        name: "guard",
        version: 1,
        ready: true,
        text: "version = 1\n[[rule]]\nid = \"tm-g1\"\nengine = \"tessera\"\nkind = \"security\"\ncheck = \"g1\"\nfix = \"f\"\n\
               failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n",
        yaml: &[],
        cases: &[("cases/tm-g1/failing/a.rs", "x\n"), ("cases/tm-g1/passing/a.rs", "x\n")],
    },
    BaselineSet {
        name: "guardneeds",
        version: 1,
        ready: true,
        text: "version = 1\n[[rule]]\nid = \"tm-n1\"\nengine = \"tessera\"\nkind = \"security\"\ncheck = \"n1\"\nfix = \"f\"\n\
               required_params = [\"types\"]\n\
               failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n",
        yaml: &[],
        cases: &[("cases/tm-n1/failing/a.rs", "x\n"), ("cases/tm-n1/passing/a.rs", "x\n")],
    },
];

/// Takes plain@1 (entry at 2:13) then guard@1 (entry at 2:24).
const TAKES_GUARD: &str = "version = 1\nbaseline = [\"plain@1\", \"guard@1\"]\n";

fn parse_secure(text: &str) -> Result<RuleFile, Vec<Problem>> {
    parse_with_sets(text, Path::new("."), SECURITY_SETS)
}

#[test]
fn baseline_security_rule_without_an_owner_is_a_problem_at_its_entry() {
    let expected = Problem {
        line: 2,
        col: 24,
        what: "a security rule needs a top-level `owner`, and the rule file names none".to_string(),
        fix: "Add a top-level `owner = \"<kit>\"`: a security finding must name its owner.".to_string(),
    };
    assert_eq!(parse_secure(TAKES_GUARD).expect_err("no owner"), vec![expected]);
}

#[test]
fn baseline_security_rule_with_an_owner_is_no_problem() {
    let file = parse_secure(&format!("{TAKES_GUARD}owner = \"core\"\n")).unwrap_or_else(|p| panic!("valid: {p:?}"));
    assert_eq!(file.owner.as_deref(), Some("core"));
    assert_eq!(ids(&file.rules), vec!["tm-p1", "tm-g1"]);
}

#[test]
fn baseline_security_rule_turned_off_needs_no_owner() {
    let text = format!("{TAKES_GUARD}[[override]]\nrule = \"tm-g1\"\noff = true\n");
    let file = parse_secure(&text).unwrap_or_else(|p| panic!("valid: {p:?}"));
    assert_eq!(ids(&file.rules), vec!["tm-p1"]);
}

#[test]
fn baseline_security_rule_not_applicable_needs_no_owner() {
    let file = parse_secure("version = 1\nbaseline = [\"guardneeds@1\"]\n").unwrap_or_else(|p| panic!("valid: {p:?}"));
    assert_eq!(ids(&file.rules), Vec::<&str>::new());
    assert_eq!(ids(&file.not_applicable_rules), vec!["tm-n1"]);
}

#[test]
fn baseline_override_with_a_bad_severity_and_an_unknown_rule_is_both_problems_in_order() {
    let text = format!("{TAKES_STANDARD}[[override]]\nrule = \"tm-none\"\nseverity = \"loud\"\n");
    let unknown = Problem {
        line: 4,
        col: 8,
        what: "override of unknown baseline rule \"tm-none\": no taken set holds it".to_string(),
        fix: UNKNOWN_OVERRIDE_FIX.to_string(),
    };
    let severity = Problem {
        line: 5,
        col: 12,
        what: "unknown severity \"loud\"".to_string(),
        fix: "Use \"error\", \"warning\" or \"watch\".".to_string(),
    };
    assert_eq!(parse(&text).expect_err("two problems"), vec![unknown, severity]);
}

#[test]
fn baseline_set_default_for_a_required_param_leaves_the_rule_not_applicable() {
    let sets = [BaselineSet {
        name: "defaulted",
        version: 1,
        ready: true,
        text: "version = 1\n[[rule]]\nid = \"tm-t1\"\nengine = \"ast-grep\"\ncheck = \"t1\"\nfix = \"f\"\n\
               params = { types = [\"X\"] }\nrequired_params = [\"types\"]\n\
               failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n",
        yaml: &[("t1", "rule: t1\n")],
        cases: &[("cases/tm-t1/failing/a.rs", "x\n"), ("cases/tm-t1/passing/a.rs", "x\n")],
    }];
    let file = parse_with_sets("version = 1\nbaseline = [\"defaulted@1\"]\n", Path::new("."), &sets)
        .unwrap_or_else(|p| panic!("valid: {p:?}"));
    assert_eq!(ids(&file.rules), Vec::<&str>::new());
    assert_eq!(ids(&file.not_applicable_rules), vec!["tm-t1"]);
    let reason = "required params not given: types; give them in the params of an [[override]] for \"tm-t1\"";
    assert_eq!(file.not_applicable_rules[0].not_applicable.as_deref(), Some(reason));
}

#[test]
fn baseline_security_copy_dropped_as_a_duplicate_needs_no_owner() {
    let sets = [
        BaselineSet {
            name: "qz",
            version: 1,
            ready: true,
            text: "version = 1\n[[rule]]\nid = \"tm-z1\"\nengine = \"ruff\"\ncheck = \"z1\"\nfix = \"f\"\n\
                   failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n",
            yaml: &[],
            cases: &[("cases/tm-z1/failing/a.rs", "x\n"), ("cases/tm-z1/passing/a.rs", "x\n")],
        },
        BaselineSet {
            name: "sz",
            version: 1,
            ready: true,
            text: "version = 1\n[[rule]]\nid = \"tm-z1\"\nengine = \"tessera\"\nkind = \"security\"\ncheck = \"z1\"\n\
                   fix = \"f\"\n\
                   failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n",
            yaml: &[],
            cases: &[("cases/tm-z1/failing/a.rs", "x\n"), ("cases/tm-z1/passing/a.rs", "x\n")],
        },
    ];
    let found = parse_with_sets("version = 1\nbaseline = [\"qz@1\", \"sz@1\"]\n", Path::new("."), &sets)
        .expect_err("the duplicate");
    let duplicate = Problem {
        line: 2,
        col: 21,
        what: "rule \"tm-z1\" is in both qz@1 and sz@1".to_string(),
        fix: "Take only one of the two sets.".to_string(),
    };
    assert_eq!(found, vec![duplicate]);
}
