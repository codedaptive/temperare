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
    for (name, expected) in [
        ("ast-grep", Engine::AstGrep),
        ("shellcheck", Engine::ShellCheck),
        ("sqlfluff", Engine::Sqlfluff),
    ] {
        let (engine, out) = read(&format!("engine = \"{name}\"\n"), 0);
        assert_eq!(engine, Some(expected), "{name:?}, {expected:?}");
        assert_eq!(out, vec![], "{name:?}, {expected:?}");
    }
}

#[test]
fn an_unknown_engine_is_none_with_a_problem_at_the_value() {
    let (engine, out) = read("id = \"r\"\nengine = \"lint\"\n", 0);
    assert_eq!(engine, None);
    let fix = format!("Use one of: {}.", Engine::ALL.map(Engine::name).join(", "));
    assert_eq!(
        out,
        vec![Problem {
            line: 2,
            col: 10,
            what: "unknown engine \"lint\"".to_string(),
            fix,
        }]
    );
    assert!(out[0].fix.starts_with("Use one of: clippy, swiftlint, "), "{:?}", out[0].fix);
    assert!(out[0].fix.ends_with(", stylelint, sqlfluff."), "{:?}", out[0].fix);
}
