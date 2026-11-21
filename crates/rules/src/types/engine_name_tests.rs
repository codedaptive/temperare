//! Unit tests for engine_name.rs: each engine's rule-file name, the round trip through `Engine::from_name`, and agreement
//! with the serde name.

use super::Engine;
use serde::Serialize;

const ALL: [Engine; 11] = [
    Engine::Clippy,
    Engine::SwiftLint,
    Engine::Ruff,
    Engine::AstGrep,
    Engine::Tessera,
    Engine::GolangciLint,
    Engine::Pylint,
    Engine::ClangTidy,
    Engine::Oclint,
    Engine::ShellCheck,
    Engine::Git,
];

#[test]
fn clippy() {
    assert_eq!(Engine::Clippy.name(), "clippy");
}

#[test]
fn swiftlint() {
    assert_eq!(Engine::SwiftLint.name(), "swiftlint");
}

#[test]
fn ruff() {
    assert_eq!(Engine::Ruff.name(), "ruff");
}

#[test]
fn ast_grep() {
    assert_eq!(Engine::AstGrep.name(), "ast-grep");
}

#[test]
fn tessera() {
    assert_eq!(Engine::Tessera.name(), "tessera");
}

#[test]
fn golangci_lint() {
    assert_eq!(Engine::GolangciLint.name(), "golangci-lint");
}

#[test]
fn pylint() {
    assert_eq!(Engine::Pylint.name(), "pylint");
}

#[test]
fn clang_tidy() {
    assert_eq!(Engine::ClangTidy.name(), "clang-tidy");
}

#[test]
fn oclint() {
    assert_eq!(Engine::Oclint.name(), "oclint");
}

#[test]
fn shellcheck() {
    assert_eq!(Engine::ShellCheck.name(), "shellcheck");
}

#[test]
fn git() {
    assert_eq!(Engine::Git.name(), "git");
}

#[test]
fn round_trips_through_from_name() {
    for engine in ALL {
        assert_eq!(Engine::from_name(engine.name()), Some(engine), "{engine:?}");
    }
}

#[test]
fn equals_the_serde_name() {
    #[derive(Serialize)]
    struct Wrap {
        engine: Engine,
    }
    for engine in ALL {
        let written = toml::to_string(&Wrap { engine }).unwrap();
        assert_eq!(written, format!("engine = \"{}\"\n", engine.name()), "{engine:?}");
    }
}
