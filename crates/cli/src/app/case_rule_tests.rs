//! ZUT for case_rule.rs: the params each case runs with, and the why when the case lacks a required param, written
//! out by hand.

use super::case_rule::case_rule;
use rules::{Engine, Rule};

/// A TOML table parsed from `text`.
fn table(text: &str) -> toml::Table {
    text.parse().unwrap()
}

/// tm-x with `required` params, `params`, failing and passing case params, and a not_applicable why to clear.
fn rule(required: &[&str], params: &str, failing: &str, passing: &str) -> Rule {
    let mut rule = Rule::new("tm-x", Engine::AstGrep, "check");
    rule.required_params = required.iter().map(|name| name.to_string()).collect();
    rule.params = table(params);
    rule.failing_params = table(failing);
    rule.passing_params = table(passing);
    rule.not_applicable = Some("set aside".to_string());
    rule
}

/// (name, rule, failing, the params it runs with or the why)
type Row = (&'static str, Rule, bool, Result<&'static str, &'static str>);

#[test]
fn case_rule_rows() {
    let rows: [Row; 6] = [
        (
            "failing case adds a param the rule lacks",
            rule(&["own_package"], "", r#"own_package = ["ledger"]"#, ""),
            true,
            Ok(r#"own_package = ["ledger"]"#),
        ),
        (
            "case wins on a shared key, the rule's other key kept",
            rule(&["own_package"], "own_package = [\"x\"]\nlimit = 3", r#"own_package = ["ledger"]"#, ""),
            true,
            Ok("own_package = [\"ledger\"]\nlimit = 3"),
        ),
        (
            "passing case uses passing_params",
            rule(&["own_package"], "", r#"own_package = ["ledger"]"#, r#"own_package = ["shop"]"#),
            false,
            Ok(r#"own_package = ["shop"]"#),
        ),
        (
            "two required params, the case gives neither",
            rule(&["a", "b"], "", "", ""),
            true,
            Err("the case gives no value for required params: a, b"),
        ),
        (
            "a default in the rule's params does not satisfy a required param",
            rule(&["own_package"], r#"own_package = ["x"]"#, "", "other = 1"),
            false,
            Err("the case gives no value for required params: own_package"),
        ),
        ("no required params, no case params", rule(&[], "limit = 3", "", ""), true, Ok("limit = 3")),
    ];
    for (name, rule, failing, expected) in rows {
        let got = case_rule(&rule, failing).map(|r| (r.params, r.not_applicable));
        let expected = expected.map(|params| (table(params), None)).map_err(str::to_string);
        assert_eq!(got, expected, "{name}");
    }
}
