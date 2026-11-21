//! ZUT for read_rule.rs.

use super::read_rule::read_rule;
use crate::{Engine, Kind, Kit, Problem, RelPath, Rule, Severity};
use toml::de::DeTable;

/// Reads the first `[[rule]]` of `text` as a repository's own rule (see `read_from`).
fn read(text: &str, params: toml::Table) -> (Option<Rule>, Vec<Problem>) {
    read_from(text, params, false)
}

/// Reads the first `[[rule]]` of `text` with two kits defined: "core" (crates/core/**) and "gen" (crates/core/gen/**,
/// tools/**); read_rule uses a kit's paths, not its scope. `from_set` as read_rule takes it.
fn read_from(text: &str, params: toml::Table, from_set: bool) -> (Option<Rule>, Vec<Problem>) {
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
    let rule = read_rule(text, item.span().start, item.get_ref().as_table().unwrap(), &kits, params, from_set, &mut out);
    (rule, out)
}

#[test]
fn a_check_starting_with_a_dash_is_rejected_at_its_string() {
    let text = "[[rule]]\nid = \"r\"\nengine = \"clippy\"\ncheck = \"-x\"\nfix = \"f\"\n";
    let (rule, out) = read(text, toml::Table::new());
    assert!(rule.is_none());
    assert_eq!(out.len(), 1, "{out:?}");
    assert_eq!((out[0].line, out[0].col), (4, 9));
    assert!(out[0].fix.contains("leading '-'"), "{}", out[0].fix);
}

#[test]
fn a_config_starting_with_a_dash_is_rejected_at_its_string() {
    let text = "[[rule]]\nid = \"r\"\nengine = \"clippy\"\ncheck = \"c\"\nconfig = \"--foo\"\nfix = \"f\"\n";
    let (rule, out) = read(text, toml::Table::new());
    assert!(rule.is_none());
    assert_eq!(out.len(), 1, "{out:?}");
    assert_eq!((out[0].line, out[0].col), (5, 10));
}

#[test]
fn an_allow_value_starting_with_a_dash_is_rejected_at_its_string() {
    let text = "[[rule]]\nid = \"r\"\nengine = \"clippy\"\ncheck = \"c\"\nparams = { allow = [\"ok\", \"-bad\"] }\nfix = \"f\"\n";
    let (rule, out) = read(text, toml::Table::new());
    assert!(rule.is_none());
    assert_eq!(out.len(), 1, "{out:?}");
    assert_eq!((out[0].line, out[0].col), (5, 27));
}

#[test]
fn dashes_inside_check_config_and_allow_values_are_accepted() {
    let text = "[[rule]]\nid = \"r\"\nengine = \"clippy\"\ncheck = \"no-stored-bool\"\nconfig = \"a-b.toml\"\nparams = { allow = [\"clippy::x-y\"] }\nfix = \"f\"\n";
    let (rule, out) = read(text, toml::Table::new());
    assert_eq!(out, vec![]);
    assert!(rule.is_some());
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
    assert_eq!(rule.enforces, None);
    assert_eq!(rule.leaves, None);
    assert!(!rule.only_with_base);
    assert_eq!(rule.required_params, Vec::<String>::new());
    assert_eq!(rule.not_applicable, None);
}

/// The one Problem of `out`, as (line, col, what, fix).
fn only_problem(out: &[Problem]) -> (usize, usize, &str, &str) {
    assert_eq!(out.len(), 1, "{out:?}");
    (out[0].line, out[0].col, out[0].what.as_str(), out[0].fix.as_str())
}

#[test]
fn enforces_leaves_and_only_with_base_are_stored_on_an_own_rule() {
    let text = "[[rule]]\nid = \"r\"\nengine = \"clippy\"\ncheck = \"c\"\nfix = \"f\"\n\
                enforces = \"No unwrap (rust skill)\"\nleaves = \"macros\"\nonly_with_base = true\n";
    let (rule, out) = read(text, toml::Table::new());
    assert_eq!(out, vec![]);
    let rule = rule.expect("no problems");
    assert_eq!(rule.enforces.as_deref(), Some("No unwrap (rust skill)"));
    assert_eq!(rule.leaves.as_deref(), Some("macros"));
    assert!(rule.only_with_base);
    assert_eq!(rule.required_params, Vec::<String>::new());
    assert_eq!(rule.not_applicable, None);
}

#[test]
fn enforces_that_is_not_a_string_is_a_wrong_type_problem() {
    let text = "[[rule]]\nid = \"r\"\nengine = \"clippy\"\ncheck = \"c\"\nfix = \"f\"\nenforces = 3\n";
    let (rule, out) = read(text, toml::Table::new());
    assert!(rule.is_none());
    assert_eq!(
        only_problem(&out),
        (6, 12, "`enforces` must be a string, not integer", "Write `enforces` as a string.")
    );
}

#[test]
fn only_with_base_that_is_not_a_boolean_is_a_wrong_type_problem() {
    let text = "[[rule]]\nid = \"r\"\nengine = \"clippy\"\ncheck = \"c\"\nfix = \"f\"\nonly_with_base = \"yes\"\n";
    let (rule, out) = read(text, toml::Table::new());
    assert!(rule.is_none());
    assert_eq!(
        only_problem(&out),
        (6, 18, "`only_with_base` must be a boolean, not string", "Write `only_with_base` as a boolean.")
    );
}

#[test]
fn required_params_are_read_on_a_set_rule() {
    let text = "[[rule]]\nid = \"tm-x\"\nengine = \"clippy\"\ncheck = \"c\"\nfix = \"f\"\n\
                required_params = [\"entities\", \"bitmaps\"]\n";
    let (rule, out) = read_from(text, toml::Table::new(), true);
    assert_eq!(out, vec![]);
    let rule = rule.expect("no problems");
    assert_eq!(rule.id, "tm-x");
    assert_eq!(rule.required_params, vec!["entities".to_string(), "bitmaps".to_string()]);
}

#[test]
fn typed_required_params_give_their_names_on_a_set_rule() {
    let text = "[[rule]]\nid = \"tm-x\"\nengine = \"clippy\"\ncheck = \"c\"\nfix = \"f\"\n\
                required_params = [\"entities\", { name = \"own_package\", type = \"strings\" }]\n";
    let (rule, out) = read_from(text, toml::Table::new(), true);
    assert_eq!(out, vec![]);
    assert_eq!(rule.expect("no problems").required_params, vec!["entities".to_string(), "own_package".to_string()]);
}

#[test]
fn required_params_in_an_own_rule_is_an_unknown_key() {
    let text = "[[rule]]\nid = \"r\"\nengine = \"clippy\"\ncheck = \"c\"\nfix = \"f\"\nrequired_params = [\"e\"]\n";
    let (rule, out) = read(text, toml::Table::new());
    assert!(rule.is_none());
    assert_eq!(
        only_problem(&out),
        (
            6,
            1,
            "unknown key `required_params` in [[rule]]",
            "Remove it, or use one of: id, engine, kind, check, config, paths, exclude, kits, outside_kits, severity, \
             fix, params, enforces, leaves, only_with_base."
        )
    );
}

#[test]
fn failing_and_passing_cases_are_accepted_on_a_set_rule() {
    let text = "[[rule]]\nid = \"tm-x\"\nengine = \"clippy\"\ncheck = \"c\"\nfix = \"f\"\n\
                failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n";
    let (rule, out) = read_from(text, toml::Table::new(), true);
    assert_eq!(out, vec![]);
    let rule = rule.expect("no problems");
    assert_eq!(rule.id, "tm-x");
    assert_eq!(rule.embedded_cases, None);
}

#[test]
fn failing_in_an_own_rule_is_an_unknown_key() {
    let text = "[[rule]]\nid = \"r\"\nengine = \"clippy\"\ncheck = \"c\"\nfix = \"f\"\n\
                failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\n";
    let (rule, out) = read(text, toml::Table::new());
    assert!(rule.is_none());
    assert_eq!(
        only_problem(&out),
        (
            6,
            1,
            "unknown key `failing` in [[rule]]",
            "Remove it, or use one of: id, engine, kind, check, config, paths, exclude, kits, outside_kits, severity, \
             fix, params, enforces, leaves, only_with_base."
        )
    );
}

#[test]
fn an_own_id_starting_tm_is_a_problem_at_the_id() {
    let text = "[[rule]]\nid = \"tm-mine\"\nengine = \"clippy\"\ncheck = \"c\"\nfix = \"f\"\n";
    let (rule, out) = read(text, toml::Table::new());
    assert!(rule.is_none());
    assert_eq!(
        only_problem(&out),
        (
            2,
            6,
            "id \"tm-mine\" starts with \"tm-\", which is kept for baseline rules",
            "Give the rule an id that does not start with \"tm-\", or change the baseline rule with an [[override]]."
        )
    );
}

#[test]
fn a_set_id_not_starting_tm_is_a_problem_at_the_id() {
    let text = "[[rule]]\nid = \"own\"\nengine = \"clippy\"\ncheck = \"c\"\nfix = \"f\"\n";
    let (rule, out) = read_from(text, toml::Table::new(), true);
    assert!(rule.is_none());
    assert_eq!(
        only_problem(&out),
        (2, 6, "baseline rule id \"own\" does not start with \"tm-\"", "Start every baseline rule's id with \"tm-\".")
    );
}

#[test]
fn a_set_id_starting_tm_is_accepted() {
    let text = "[[rule]]\nid = \"tm-x\"\nengine = \"clippy\"\ncheck = \"c\"\nfix = \"f\"\n";
    let (rule, out) = read_from(text, toml::Table::new(), true);
    assert_eq!(out, vec![]);
    assert_eq!(rule.expect("no problems").id, "tm-x");
}

#[test]
fn an_own_id_starting_tmx_is_accepted() {
    let text = "[[rule]]\nid = \"tmx-a\"\nengine = \"clippy\"\ncheck = \"c\"\nfix = \"f\"\n";
    let (rule, out) = read(text, toml::Table::new());
    assert_eq!(out, vec![]);
    assert_eq!(rule.expect("no problems").id, "tmx-a");
}

#[test]
fn a_malformed_tm_id_has_only_the_malformed_problem() {
    let text = "[[rule]]\nid = \"tm-A\"\nengine = \"clippy\"\ncheck = \"c\"\nfix = \"f\"\n";
    let (rule, out) = read(text, toml::Table::new());
    assert!(rule.is_none());
    assert_eq!(
        only_problem(&out),
        (
            2,
            6,
            "malformed id \"tm-A\": an id is one or more of a-z, 0-9 and '-'",
            "Write the id in lower-case letters, digits and dashes, e.g. \"no-stored-bool\"."
        )
    );
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
