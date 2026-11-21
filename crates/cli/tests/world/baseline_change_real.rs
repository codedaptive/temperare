//! RIT: `run_baseline_test` runs a change-only git rule's cases against their real bases.

use cli::{run_baseline_test, BaselineArgs, BaselineWorld};
use rules::{CaseFile, Engine, ExpectedFinding, Rule, RuleCases};

#[test]
fn baseline_change_real_holds_test_ratio_against_each_base() {
    let mut rule = Rule::new("test-ratio", Engine::Git, "test_ratio");
    rule.only_with_base = true;
    rule.params.insert("max_ratio".into(), toml::Value::Float(1.5));
    rule.params.insert("floor".into(), toml::Value::Integer(1));
    rule.params.insert("test_paths".into(), toml::Value::Array(vec![toml::Value::String("tests/**".into())]));
    rule.embedded_cases = Some(RuleCases {
        failing: vec![CaseFile {
            path: "tests/a_tests.rs".into(),
            text: "base test\none\ntwo\nthree\nfour\nfive\n",
        }],
        passing: vec![
            CaseFile { path: "src/lib.rs".into(), text: "one\ntwo\nthree\nfour\n" },
            CaseFile { path: "tests/a_tests.rs".into(), text: "base test\n" },
        ],
        expected: ExpectedFinding { rule_id: "test-ratio".into(), path: "tests/a_tests.rs".into(), line: 2 },
    });
    let rules_of = |_: &str| Ok(vec![rule.clone()]);
    let world = BaselineWorld { rules_of: &rules_of, ..BaselineWorld::real() };
    let args = BaselineArgs { set: Some("const@1".into()), json: false };
    let mut out = Vec::new();
    let mut err = Vec::new();
    let exit = run_baseline_test(&args, &world, &mut out, &mut err);
    let text = String::from_utf8(out).unwrap();
    let stderr = String::from_utf8(err).unwrap();
    assert_eq!(exit, 0, "stdout:\n{text}\nstderr:\n{stderr}");
    assert!(text.lines().any(|line| line == "const@1 test-ratio (git): holds"), "stdout:\n{text}\nstderr:\n{stderr}");
    assert!(text.lines().any(|line| line == "1 rules: 1 hold, 0 broken, 0 not available, 0 not applicable"), "stdout:\n{text}\nstderr:\n{stderr}");
}
