//! ZUT for standard_1.rs.

use super::load_set::load_set;
use super::standard_1::STANDARD_1;
use crate::{Engine, Kind, Rule, Severity};

const Q: Kind = Kind::Quality;
const W: Severity = Severity::Warning;

/// One rule as the set file writes it: id, engine name, check, kind, severity, params, only_with_base, and the path
/// and line its failing case must give.
type Row<'a> = (&'a str, &'a str, &'a str, Kind, Severity, toml::Table, bool, &'a str, usize);

/// A params table holding the integer `value` under `name`.
fn param(name: &str, value: i64) -> toml::Table {
    toml::Table::from_iter([(name.to_string(), toml::Value::Integer(value))])
}

fn none() -> toml::Table {
    toml::Table::new()
}

fn rules() -> Vec<Rule> {
    load_set(&STANDARD_1).unwrap_or_else(|p| panic!("standard@1 has problems: {p:?}")).0
}

/// The 53 rules in S-55's order (dead code, deprecated, suppressions, then the rest of SPEC-baseline's standard list,
/// then what a change adds), as the set file writes them.
fn expected_rows() -> Vec<Row<'static>> {
    vec![
        ("tm-rust-dead-code", "clippy", "dead_code", Q, W, none(), false, "src/lib.rs", 8),
        ("tm-rust-unused-imports", "clippy", "unused_imports", Q, W, none(), false, "src/lib.rs", 4),
        ("tm-rust-unused-variables", "clippy", "unused_variables", Q, W, none(), false, "src/lib.rs", 5),
        ("tm-python-unused-import", "ruff", "F401", Q, W, none(), false, "settings.py", 4),
        ("tm-python-unused-variable", "ruff", "F841", Q, W, none(), false, "orders.py", 7),
        ("tm-rust-deprecated", "clippy", "deprecated", Q, W, none(), false, "src/lib.rs", 5),
        ("tm-rust-suppression", "ast-grep", "ast-grep/tm-rust-suppression.yml", Q, W, none(), false, "src/lib.rs", 2),
        ("tm-swift-suppression", "ast-grep", "ast-grep/tm-swift-suppression.yml", Q, W, none(), false, "a.swift", 2),
        ("tm-python-suppression", "ast-grep", "ast-grep/tm-python-suppression.yml", Q, W, none(), false, "a.py", 5),
        ("tm-javascript-suppression", "ast-grep", "ast-grep/tm-javascript-suppression.yml", Q, W, none(), false, "a.js", 2),
        ("tm-typescript-suppression", "ast-grep", "ast-grep/tm-typescript-suppression.yml", Q, W, none(), false, "a.ts", 2),
        ("tm-go-suppression", "ast-grep", "ast-grep/tm-go-suppression.yml", Q, W, none(), false, "a.go", 5),
        ("tm-c-suppression", "ast-grep", "ast-grep/tm-c-suppression.yml", Q, W, none(), false, "a.c", 2),
        ("tm-cpp-suppression", "ast-grep", "ast-grep/tm-cpp-suppression.yml", Q, W, none(), false, "a.cpp", 2),
        ("tm-rust-allow-without-reason", "clippy", "clippy::allow_attributes_without_reason", Q, W, none(), false, "src/lib.rs", 1),
        ("tm-swift-blanket-disable", "swiftlint", "blanket_disable_command", Q, W, none(), false, "a.swift", 1),
        ("tm-python-blanket-noqa", "ruff", "PGH004", Q, W, none(), false, "a.py", 1),
        ("tm-swift-complexity", "swiftlint", "cyclomatic_complexity", Q, W, param("max_complexity", 10), false, "Status.swift", 2),
        ("tm-python-complexity", "ruff", "C901", Q, W, param("max_complexity", 10), false, "status.py", 4),
        ("tm-rust-nesting", "clippy", "clippy::excessive_nesting", Q, W, param("max_nesting", 6), false, "src/lib.rs", 21),
        ("tm-rust-dbg", "clippy", "clippy::dbg_macro", Q, W, none(), false, "src/lib.rs", 5),
        ("tm-python-print", "ruff", "T201", Q, W, none(), false, "orders.py", 7),
        ("tm-python-bare-except", "ruff", "E722", Q, W, none(), false, "ports.py", 8),
        ("tm-python-blind-except", "ruff", "BLE001", Q, W, none(), false, "ports.py", 8),
        ("tm-python-commented-out-code", "ruff", "ERA001", Q, W, none(), false, "orders.py", 7),
        ("tm-change-adds-sleep-rust", "ast-grep", "ast-grep/tm-change-adds-sleep-rust.yml", Q, W, none(), true, "worker.rs", 11),
        ("tm-change-adds-sleep-swift", "ast-grep", "ast-grep/tm-change-adds-sleep-swift.yml", Q, W, none(), true, "Worker.swift", 6),
        ("tm-change-adds-sleep-python", "ast-grep", "ast-grep/tm-change-adds-sleep-python.yml", Q, W, none(), true, "worker.py", 9),
        ("tm-change-adds-sleep-go", "ast-grep", "ast-grep/tm-change-adds-sleep-go.yml", Q, W, none(), true, "worker.go", 9),
        ("tm-change-adds-sleep-javascript", "ast-grep", "ast-grep/tm-change-adds-sleep-javascript.yml", Q, W, none(), true, "worker.js", 4),
        ("tm-change-adds-sleep-typescript", "ast-grep", "ast-grep/tm-change-adds-sleep-typescript.yml", Q, W, none(), true, "worker.ts", 6),
        ("tm-change-adds-retry-rust", "ast-grep", "ast-grep/tm-change-adds-retry-rust.yml", Q, W, none(), true, "send.rs", 7),
        ("tm-change-adds-retry-swift", "ast-grep", "ast-grep/tm-change-adds-retry-swift.yml", Q, W, none(), true, "Send.swift", 6),
        ("tm-change-adds-retry-python", "ast-grep", "ast-grep/tm-change-adds-retry-python.yml", Q, W, none(), true, "send.py", 7),
        ("tm-change-adds-retry-go", "ast-grep", "ast-grep/tm-change-adds-retry-go.yml", Q, W, none(), true, "send.go", 11),
        ("tm-change-adds-retry-javascript", "ast-grep", "ast-grep/tm-change-adds-retry-javascript.yml", Q, W, none(), true, "send.js", 6),
        ("tm-change-adds-retry-typescript", "ast-grep", "ast-grep/tm-change-adds-retry-typescript.yml", Q, W, none(), true, "send.ts", 6),
        ("tm-change-adds-skipped-test-rust", "ast-grep", "ast-grep/tm-change-adds-skipped-test-rust.yml", Q, W, none(), true, "parse_tests.rs", 11),
        ("tm-change-adds-skipped-test-swift", "ast-grep", "ast-grep/tm-change-adds-skipped-test-swift.yml", Q, W, none(), true, "ParseTests.swift", 11),
        ("tm-change-adds-skipped-test-python", "ast-grep", "ast-grep/tm-change-adds-skipped-test-python.yml", Q, W, none(), true, "test_parse.py", 13),
        ("tm-change-adds-skipped-test-go", "ast-grep", "ast-grep/tm-change-adds-skipped-test-go.yml", Q, W, none(), true, "parse_test.go", 20),
        ("tm-change-adds-skipped-test-javascript", "ast-grep", "ast-grep/tm-change-adds-skipped-test-javascript.yml", Q, W, none(), true, "parse.test.js", 10),
        ("tm-change-adds-skipped-test-typescript", "ast-grep", "ast-grep/tm-change-adds-skipped-test-typescript.yml", Q, W, none(), true, "parse.test.ts", 10),
        ("tm-change-adds-weaker-assertion-rust", "ast-grep", "ast-grep/tm-change-adds-weaker-assertion-rust.yml", Q, W, none(), true, "parse_tests.rs", 7),
        ("tm-change-adds-weaker-assertion-swift", "ast-grep", "ast-grep/tm-change-adds-weaker-assertion-swift.yml", Q, W, none(), true, "ParseTests.swift", 8),
        ("tm-change-adds-weaker-assertion-python", "ast-grep", "ast-grep/tm-change-adds-weaker-assertion-python.yml", Q, W, none(), true, "test_parse.py", 7),
        ("tm-change-adds-weaker-assertion-javascript", "ast-grep", "ast-grep/tm-change-adds-weaker-assertion-javascript.yml", Q, W, none(), true, "parse.test.js", 7),
        ("tm-change-adds-weaker-assertion-typescript", "ast-grep", "ast-grep/tm-change-adds-weaker-assertion-typescript.yml", Q, W, none(), true, "parse.test.ts", 7),
        ("tm-change-adds-wider-catch-rust", "ast-grep", "ast-grep/tm-change-adds-wider-catch-rust.yml", Q, W, none(), true, "run.rs", 6),
        ("tm-change-adds-wider-catch-swift", "ast-grep", "ast-grep/tm-change-adds-wider-catch-swift.yml", Q, W, none(), true, "Settings.swift", 8),
        ("tm-change-adds-wider-catch-python", "ast-grep", "ast-grep/tm-change-adds-wider-catch-python.yml", Q, W, none(), true, "config.py", 9),
        ("tm-change-adds-wider-catch-javascript", "ast-grep", "ast-grep/tm-change-adds-wider-catch-javascript.yml", Q, W, none(), true, "config.js", 8),
        ("tm-change-adds-wider-catch-typescript", "ast-grep", "ast-grep/tm-change-adds-wider-catch-typescript.yml", Q, W, none(), true, "config.ts", 8),
    ]
}

#[test]
fn standard_1_rules_come_back_in_set_order_as_written() {
    let rules = rules();
    let found: Vec<Row> = rules
        .iter()
        .map(|rule| {
            let expected = &rule.cases().unwrap_or_else(|| panic!("{} has no cases", rule.id)).expected;
            (
                rule.id.as_str(),
                rule.engine.name(),
                rule.check.as_str(),
                rule.kind,
                rule.severity,
                rule.params.clone(),
                rule.only_with_base,
                expected.path.as_str(),
                expected.line,
            )
        })
        .collect();
    assert_eq!(found, expected_rows());
}

#[test]
fn every_standard_1_rule_carries_both_cases_for_itself() {
    let rules = rules();
    let found: Vec<_> = rules
        .iter()
        .map(|rule| {
            let cases = rule.cases().map(|c| (!c.failing.is_empty(), !c.passing.is_empty(), c.expected.rule_id.as_str()));
            (rule.id.as_str(), cases)
        })
        .collect();
    let expected: Vec<_> = rules.iter().map(|rule| (rule.id.as_str(), Some((true, true, rule.id.as_str())))).collect();
    assert_eq!(found, expected);
}

#[test]
fn every_standard_1_ast_grep_rule_and_only_those_carry_yaml() {
    let rules = rules();
    let found: Vec<_> = rules.iter().map(|rule| (rule.id.as_str(), rule.embedded_yaml().is_some())).collect();
    let expected: Vec<_> = rules.iter().map(|rule| (rule.id.as_str(), rule.engine == Engine::AstGrep)).collect();
    assert_eq!(found, expected);
}
