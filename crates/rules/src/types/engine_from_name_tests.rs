//! Unit tests for engine_from_name.rs: the five rule-file names, and None for anything else.

use super::Engine;

#[test]
fn clippy() {
    assert_eq!(Engine::from_name("clippy"), Some(Engine::Clippy));
}

#[test]
fn swiftlint() {
    assert_eq!(Engine::from_name("swiftlint"), Some(Engine::SwiftLint));
}

#[test]
fn ruff() {
    assert_eq!(Engine::from_name("ruff"), Some(Engine::Ruff));
}

#[test]
fn ast_grep() {
    assert_eq!(Engine::from_name("ast-grep"), Some(Engine::AstGrep));
}

#[test]
fn tessera() {
    assert_eq!(Engine::from_name("tessera"), Some(Engine::Tessera));
}

#[test]
fn unknown_names_are_none() {
    for name in ["eslint", "", "Clippy", "ast_grep", "SwiftLint", " ruff"] {
        assert_eq!(Engine::from_name(name), None, "{name:?}");
    }
}
