//! Unit tests for engine_value.rs.

use super::engine_value::engine_value;
use crate::{Engine, Problem};
use toml::de::DeTable;

/// `engine_value` on the whole of `text` as one table, the header taken to start at byte `at`.
fn read(text: &str, at: usize) -> (Option<Engine>, Vec<Problem>) {
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    (engine_value(text, doc.get_ref(), at, &mut out), out)
}

#[test]
fn an_absent_engine_is_none_with_a_problem_at_the_header() {
    let (engine, out) = read("id = \"r\"\n", 0);
    assert_eq!(engine, None);
    assert_eq!(
        out,
        vec![Problem {
            line: 1,
            col: 1,
            what: "[[rule]] is missing `engine`".to_string(),
            fix: "Add `engine = \"...\"` to this [[rule]].".to_string(),
        }]
    );
}

#[test]
fn a_known_engine_is_read() {
    let (engine, out) = read("engine = \"ast-grep\"\n", 0);
    assert_eq!(engine, Some(Engine::AstGrep));
    assert_eq!(out, vec![]);
}

#[test]
fn an_unknown_engine_is_none_with_a_problem_at_the_value() {
    let (engine, out) = read("id = \"r\"\nengine = \"lint\"\n", 0);
    assert_eq!(engine, None);
    assert_eq!(
        out,
        vec![Problem {
            line: 2,
            col: 10,
            what: "unknown engine \"lint\"".to_string(),
            fix: "Use one of: clippy, swiftlint, ruff, ast-grep, tessera, golangci-lint, pylint, clang-tidy, oclint, shellcheck, git."
                .to_string(),
        }]
    );
}

#[test]
fn a_new_engine_name_is_read() {
    let (engine, out) = read("engine = \"shellcheck\"\n", 0);
    assert_eq!(engine, Some(Engine::ShellCheck));
    assert_eq!(out, vec![]);
}
