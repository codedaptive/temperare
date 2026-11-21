//! ZUT for house_1.rs.

use super::house_1::HOUSE_1;
use super::load_set::load_set;
use crate::{Engine, Kind, Rule, Severity};

const Q: Kind = Kind::Quality;
const E: Severity = Severity::Error;
const W: Severity = Severity::Warning;

/// One rule as the set file writes it: id, engine name, check, kind, severity, params, required_params,
/// only_with_base, and the path and line its failing case must give.
pub(super) type Row<'a> = (&'a str, &'a str, &'a str, Kind, Severity, toml::Table, Vec<String>, bool, &'a str, usize);

fn table<const N: usize>(entries: [(&str, toml::Value); N]) -> toml::Table {
    entries.into_iter().map(|(name, value)| (name.to_string(), value)).collect()
}

fn int(value: i64) -> toml::Value {
    toml::Value::Integer(value)
}

fn text(value: &str) -> toml::Value {
    toml::Value::String(value.to_string())
}

fn texts(values: &[&str]) -> toml::Value {
    toml::Value::Array(values.iter().map(|value| text(value)).collect())
}

fn none() -> toml::Table {
    toml::Table::new()
}

fn needs(names: &[&str]) -> Vec<String> {
    names.iter().map(|name| name.to_string()).collect()
}

/// The test-ratio rule's params: a limit of 1.5 lines of test per line of code past 40 lines, and its per-language
/// test paths.
fn test_ratio() -> toml::Table {
    let paths = ["**/tests/**", "**/*_tests.rs", "**/test_*.py", "**/*_test.py", "**/*Tests/**", "**/*Tests.swift"];
    table([("max_ratio", toml::Value::Float(1.5)), ("floor", int(40)), ("test_paths", texts(&paths))])
}

/// A setup-heavier rule's params: the count mode and the language's shared setup files.
fn setup_vs_tests(shared_setup: &[&str]) -> toml::Table {
    table([("count", text("setup-vs-tests")), ("shared_setup", texts(shared_setup))])
}

fn rules() -> Vec<Rule> {
    load_set(&HOUSE_1).unwrap_or_else(|p| panic!("house@1 has problems: {p:?}")).0
}

/// The 36 rules: 27 rules for the code, grouped by practice, then the nine rules of the four warnings that keep
/// tests lean, as the set file writes them.
pub(super) fn expected_rows() -> Vec<Row<'static>> {
    vec![
        ("tm-house-no-unwrap-rust", "clippy", "clippy::unwrap_used", Q, W, none(), needs(&[]), false, "src/lib.rs", 5),
        ("tm-house-no-expect-rust", "clippy", "clippy::expect_used", Q, W, none(), needs(&[]), false, "src/lib.rs", 7),
        ("tm-house-no-unsafe-rust", "clippy", "unsafe_code", Q, W, none(), needs(&[]), false, "src/lib.rs", 8),
        ("tm-house-no-print-stdout-rust", "clippy", "clippy::print_stdout", Q, W, none(), needs(&[]), false, "src/lib.rs", 6),
        ("tm-house-no-todo-python", "ruff", "FIX002", Q, W, none(), needs(&[]), false, "shipping.py", 5),
        ("tm-house-no-todo-swift", "swiftlint", "todo", Q, W, none(), needs(&[]), false, "DeliveryFee.swift", 3),
        ("tm-house-no-todo-rust", "ast-grep", "ast-grep/tm-house-no-todo-rust.yml", Q, W, none(), needs(&[]), false, "worker.rs", 5),
        ("tm-house-no-force-cast-swift", "swiftlint", "force_cast", Q, W, none(), needs(&[]), false, "UserName.swift", 3),
        ("tm-house-no-force-try-swift", "swiftlint", "force_try", Q, W, none(), needs(&[]), false, "SettingsText.swift", 5),
        ("tm-house-no-force-unwrapping-swift", "swiftlint", "force_unwrapping", Q, W, none(), needs(&[]), false, "HealthURL.swift", 5),
        ("tm-house-no-as-conversions-rust", "clippy", "clippy::as_conversions", Q, W, none(), needs(&[]), false, "src/lib.rs", 5),
        ("tm-house-no-assign-then-return-python", "ruff", "RET504", Q, W, none(), needs(&[]), false, "names.py", 6),
        ("tm-house-no-let-and-return-rust", "clippy", "clippy::let_and_return", Q, W, none(), needs(&[]), false, "src/lib.rs", 6),
        ("tm-house-arg-type-hints-python", "ruff", "ANN001", Q, W, none(), needs(&[]), false, "ports.py", 4),
        ("tm-house-return-type-hints-python", "ruff", "ANN201", Q, W, none(), needs(&[]), false, "ports.py", 4),
        ("tm-house-pathlib-python", "ruff", "PTH", Q, W, none(), needs(&[]), false, "config.py", 7),
        ("tm-house-one-public-symbol-rust", "ast-grep", "ast-grep/tm-house-one-public-symbol-rust.yml", Q, W, none(), needs(&[]), false, "report.rs", 7),
        ("tm-house-one-declaration-swift", "swiftlint", "one_declaration_per_file", Q, W, none(), needs(&[]), false, "Circle.swift", 7),
        ("tm-house-file-name-swift", "swiftlint", "file_name", Q, W, none(), needs(&[]), false, "Orders.swift", 1),
        ("tm-house-file-length-swift", "swiftlint", "file_length", Q, W, table([("max_lines", int(150))]), needs(&[]), false, "ShippingZone.swift", 151),
        ("tm-house-no-stored-bool-swift", "ast-grep", "ast-grep/tm-house-no-stored-bool-swift.yml", Q, E, none(), needs(&["entity_types"]), false, "Drawer.swift", 4),
        ("tm-house-no-stored-bool-rust", "ast-grep", "ast-grep/tm-house-no-stored-bool-rust.yml", Q, E, none(), needs(&["entity_types"]), false, "drawer.rs", 4),
        ("tm-house-bitmap-int64-swift", "ast-grep", "ast-grep/tm-house-bitmap-int64-swift.yml", Q, E, none(), needs(&["entity_types"]), false, "Drawer.swift", 4),
        ("tm-house-bitmap-int64-rust", "ast-grep", "ast-grep/tm-house-bitmap-int64-rust.yml", Q, E, none(), needs(&["entity_types"]), false, "drawer.rs", 4),
        ("tm-house-leading-trailing-swift", "ast-grep", "ast-grep/tm-house-leading-trailing-swift.yml", Q, E, none(), needs(&[]), false, "SettingsRow.swift", 8),
        ("tm-house-display-text-literal-swift", "ast-grep", "ast-grep/tm-house-display-text-literal-swift.yml", Q, E, none(), needs(&[]), false, "SettingsView.swift", 6),
        ("tm-house-swift-testing-swift", "ast-grep", "ast-grep/tm-house-swift-testing-swift.yml", Q, W, none(), needs(&[]), false, "ParserTests.swift", 1),
        ("tm-house-test-ratio", "git", "test_ratio", Q, W, test_ratio(), needs(&[]), true, "tests/parse.rs", 1),
        ("tm-house-test-asserts-sentence-rust", "ast-grep", "ast-grep/tm-house-test-asserts-sentence-rust.yml", Q, W, none(), needs(&[]), true, "describe_tests.rs", 7),
        ("tm-house-test-asserts-sentence-python", "ast-grep", "ast-grep/tm-house-test-asserts-sentence-python.yml", Q, W, none(), needs(&[]), true, "tests/test_describe.py", 5),
        ("tm-house-test-asserts-sentence-swift", "ast-grep", "ast-grep/tm-house-test-asserts-sentence-swift.yml", Q, W, none(), needs(&[]), true, "DescribeTests.swift", 8),
        ("tm-house-unit-test-fakes-own-kit-rust", "ast-grep", "ast-grep/tm-house-unit-test-fakes-own-kit-rust.yml", Q, W, none(), needs(&[]), true, "src/store_tests.rs", 7),
        ("tm-house-unit-test-fakes-own-kit-python", "ast-grep", "ast-grep/tm-house-unit-test-fakes-own-kit-python.yml", Q, W, none(), needs(&["own_package"]), true, "tests/unit/test_store.py", 7),
        ("tm-house-test-setup-heavier-rust", "ast-grep", "ast-grep/tm-house-test-setup-heavier-rust.yml", Q, W, setup_vs_tests(&["**/tests/common/**"]), needs(&[]), true, "tests/parse.rs", 1),
        ("tm-house-test-setup-heavier-python", "ast-grep", "ast-grep/tm-house-test-setup-heavier-python.yml", Q, W, setup_vs_tests(&["**/conftest.py", "**/tests/**/helpers.py"]), needs(&[]), true, "tests/test_parse.py", 1),
        ("tm-house-test-setup-heavier-swift", "ast-grep", "ast-grep/tm-house-test-setup-heavier-swift.yml", Q, W, setup_vs_tests(&["**/*Tests/Support/**"]), needs(&[]), true, "ParseTests.swift", 1),
    ]
}

#[test]
fn house_1_rules_come_back_in_set_order_as_written() {
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
                rule.required_params.clone(),
                rule.only_with_base,
                expected.path.as_str(),
                expected.line,
            )
        })
        .collect();
    assert_eq!(found, expected_rows());
}

#[test]
fn every_house_1_rule_carries_both_cases_for_itself() {
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
fn every_house_1_ast_grep_rule_and_only_those_carry_yaml() {
    let rules = rules();
    let found: Vec<_> = rules.iter().map(|rule| (rule.id.as_str(), rule.embedded_yaml().is_some())).collect();
    let expected: Vec<_> = rules.iter().map(|rule| (rule.id.as_str(), rule.engine == Engine::AstGrep)).collect();
    assert_eq!(found, expected);
}
