//! ZUT for read_rule.rs.

use super::read_rule::read_rule;
use crate::{Engine, Kind, Kit, Problem, RelPath, Rule, Severity};
use toml::de::DeTable;

/// Reads the first `[[rule]]` of `text` with two kits defined: "core" (crates/core/**) and "gen" (crates/core/gen/**,
/// tools/**); read_rule uses a kit's paths, not its scope.
fn read(text: &str, params: toml::Table) -> (Option<Rule>, Vec<Problem>) {
    let doc = DeTable::parse(text).unwrap();
    let item = &doc.get_ref().get("rule").unwrap().get_ref().as_array().unwrap()[0];
    let kit = |name: &str, paths: &[&str]| Kit {
        name: name.to_string(),
        paths: paths.iter().map(|p| p.to_string()).collect(),
        owner: None,
        scope: None,
    };
    let kits = vec![kit("core", &["crates/core/**"]), kit("gen", &["crates/core/gen/**", "tools/**"])];
    let mut out = Vec::new();
    let rule = read_rule(text, item.span().start, item.get_ref().as_table().unwrap(), &kits, params, &mut out);
    (rule, out)
}

#[test]
fn a_full_rule_is_read_with_its_scope() {
    let text = "[[rule]]\nid = \"no-bool\"\nengine = \"ruff\"\ncheck = \"E501\"\nconfig = \"ruff.toml\"\n\
                paths = [\"**/*.py\"]\nexclude = [\"**/gen/**\"]\nkits = [\"core\"]\nseverity = \"warning\"\nfix = \"Wrap it.\"\n";
    let params = toml::Table::from_iter([("n".to_string(), toml::Value::Integer(3))]);
    let (rule, out) = read(text, params.clone());
    assert_eq!(out, vec![]);
    let rule = rule.expect("no problems");
    assert_eq!(rule.id, "no-bool");
    assert_eq!(rule.engine, Engine::Ruff);
    assert_eq!(rule.check, "E501");
    assert_eq!(rule.config.as_deref(), Some("ruff.toml"));
    assert_eq!(rule.kits, vec!["core".to_string()]);
    assert_eq!(rule.severity, Severity::Warning);
    assert_eq!(rule.fix, "Wrap it.");
    assert_eq!(rule.params, params);
    assert!(rule.applies(&RelPath::new("crates/core/a.py")));
    assert!(!rule.applies(&RelPath::new("crates/core/gen/a.py")));
    assert!(!rule.applies(&RelPath::new("web/a.py")));
}

#[test]
fn defaults_are_error_severity_no_config_and_every_file() {
    let text = "[[rule]]\nid = \"r\"\nengine = \"clippy\"\ncheck = \"clippy::all\"\nfix = \"f\"\n";
    let (rule, out) = read(text, toml::Table::new());
    assert_eq!(out, vec![]);
    let rule = rule.expect("no problems");
    assert_eq!(rule.kind, Kind::Quality);
    assert_eq!(rule.severity, Severity::Error);
    assert_eq!(rule.config, None);
    assert!(rule.params.is_empty());
    assert!(rule.applies(&RelPath::new("any/where.rs")));
}

#[test]
fn kind_security_is_read() {
    let text = "[[rule]]\nid = \"r\"\nengine = \"tessera\"\nkind = \"security\"\ncheck = \"c\"\nfix = \"f\"\n";
    let (rule, out) = read(text, toml::Table::new());
    assert_eq!(out, vec![]);
    assert_eq!(rule.expect("no problems").kind, Kind::Security);
}

#[test]
fn kind_quality_written_out_is_read() {
    let text = "[[rule]]\nid = \"r\"\nengine = \"ruff\"\nkind = \"quality\"\ncheck = \"c\"\nfix = \"f\"\n";
    let (rule, out) = read(text, toml::Table::new());
    assert_eq!(out, vec![]);
    assert_eq!(rule.expect("no problems").kind, Kind::Quality);
}

#[test]
fn an_unknown_kind_leaves_the_rule_out_with_a_problem_at_the_value() {
    let text = "[[rule]]\nid = \"r\"\nengine = \"ruff\"\nkind = \"style\"\ncheck = \"c\"\nfix = \"f\"\n";
    let (rule, out) = read(text, toml::Table::new());
    assert!(rule.is_none());
    assert_eq!(out.len(), 1, "{out:?}");
    assert_eq!((out[0].line, out[0].col), (4, 8));
    assert!(out[0].what.contains("\"style\""), "{}", out[0].what);
    assert!(out[0].fix.contains("quality") && out[0].fix.contains("security"), "{}", out[0].fix);
}

#[test]
fn a_kind_that_is_not_a_string_is_a_wrong_type_problem_at_the_value() {
    let text = "[[rule]]\nid = \"r\"\nengine = \"ruff\"\nkind = 1\ncheck = \"c\"\nfix = \"f\"\n";
    let (rule, out) = read(text, toml::Table::new());
    assert!(rule.is_none());
    assert_eq!(out.len(), 1, "{out:?}");
    assert_eq!((out[0].line, out[0].col), (4, 8));
    assert!(out[0].what.contains("kind"), "{}", out[0].what);
}

#[test]
fn a_rule_with_a_problem_is_left_out_and_every_problem_is_reported() {
    // No `check` (header, 1:1), unknown engine (3:10), undefined kit (4:9).
    let text = "[[rule]]\nid = \"r\"\nengine = \"lint\"\nkits = [\"nope\"]\nfix = \"f\"\n";
    let (rule, out) = read(text, toml::Table::new());
    assert!(rule.is_none());
    let mut found: Vec<(usize, usize)> = out.iter().map(|p| (p.line, p.col)).collect();
    found.sort();
    assert_eq!(found, vec![(1, 1), (3, 10), (4, 9)]);
}

#[test]
fn params_that_is_not_a_table_is_a_problem() {
    let text = "[[rule]]\nid = \"r\"\nengine = \"clippy\"\ncheck = \"c\"\nfix = \"f\"\nparams = 3\n";
    let (rule, out) = read(text, toml::Table::new());
    assert!(rule.is_none());
    assert_eq!(out.len(), 1);
    assert_eq!((out[0].line, out[0].col), (6, 10));
    assert!(out[0].what.contains("params"), "{}", out[0].what);
}

#[test]
fn outside_kits_alone_leaves_out_the_files_of_those_kits() {
    let text = "[[rule]]\nid = \"r\"\nengine = \"ruff\"\ncheck = \"c\"\noutside_kits = [\"core\"]\nfix = \"f\"\n";
    let (rule, out) = read(text, toml::Table::new());
    assert_eq!(out, vec![]);
    let rule = rule.expect("no problems");
    assert_eq!(rule.outside_kits, vec!["core".to_string()]);
    assert!(!rule.applies(&RelPath::new("crates/core/a.rs")));
    assert!(rule.applies(&RelPath::new("web/a.rs")));
}

#[test]
fn outside_kits_with_paths_leaves_out_a_matching_file_in_an_outside_kit() {
    let text = "[[rule]]\nid = \"r\"\nengine = \"ruff\"\ncheck = \"c\"\npaths = [\"**/*.rs\"]\n\
                outside_kits = [\"core\"]\nfix = \"f\"\n";
    let (rule, out) = read(text, toml::Table::new());
    assert_eq!(out, vec![]);
    let rule = rule.expect("no problems");
    assert!(!rule.applies(&RelPath::new("crates/core/a.rs")));
    assert!(rule.applies(&RelPath::new("web/a.rs")));
    assert!(!rule.applies(&RelPath::new("web/a.py")));
}

#[test]
fn outside_kits_with_kits_leaves_out_a_file_in_both() {
    let text = "[[rule]]\nid = \"r\"\nengine = \"ruff\"\ncheck = \"c\"\nkits = [\"core\"]\n\
                outside_kits = [\"gen\"]\nfix = \"f\"\n";
    let (rule, out) = read(text, toml::Table::new());
    assert_eq!(out, vec![]);
    let rule = rule.expect("no problems");
    assert!(!rule.applies(&RelPath::new("crates/core/gen/a.rs")));
    assert!(rule.applies(&RelPath::new("crates/core/a.rs")));
    assert!(!rule.applies(&RelPath::new("tools/x.rs")));
}

#[test]
fn an_undefined_outside_kit_leaves_the_rule_out_with_a_problem_at_its_string() {
    let text = "[[rule]]\nid = \"r\"\nengine = \"ruff\"\noutside_kits = [\"nope\"]\ncheck = \"c\"\nfix = \"f\"\n";
    let (rule, out) = read(text, toml::Table::new());
    assert!(rule.is_none());
    assert_eq!(out.len(), 1, "{out:?}");
    assert_eq!((out[0].line, out[0].col), (4, 17));
    assert!(out[0].what.contains("\"nope\""), "{}", out[0].what);
    assert!(out[0].fix.contains("`outside_kits`"), "{}", out[0].fix);
}
