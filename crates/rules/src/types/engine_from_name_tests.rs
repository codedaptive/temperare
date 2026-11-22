//! ZUT for engine_from_name.rs: the fourteen rule-file names, and None for anything else.

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
fn golangci_lint() {
    assert_eq!(Engine::from_name("golangci-lint"), Some(Engine::GolangciLint));
}

#[test]
fn pylint() {
    assert_eq!(Engine::from_name("pylint"), Some(Engine::Pylint));
}

#[test]
fn clang_tidy() {
    assert_eq!(Engine::from_name("clang-tidy"), Some(Engine::ClangTidy));
}

#[test]
fn shellcheck() {
    assert_eq!(Engine::from_name("shellcheck"), Some(Engine::ShellCheck));
}

#[test]
fn git() {
    assert_eq!(Engine::from_name("git"), Some(Engine::Git));
}

#[test]
fn vulture() {
    assert_eq!(Engine::from_name("vulture"), Some(Engine::Vulture));
}

#[test]
fn periphery() {
    assert_eq!(Engine::from_name("periphery"), Some(Engine::Periphery));
}

#[test]
fn gitleaks() {
    assert_eq!(Engine::from_name("gitleaks"), Some(Engine::Gitleaks));
}

#[test]
fn cargo_audit() {
    assert_eq!(Engine::from_name("cargo-audit"), Some(Engine::CargoAudit));
}

#[test]
fn unknown_names_are_none() {
    for name in [
        "eslint",
        "",
        "Clippy",
        "ast_grep",
        "SwiftLint",
        " ruff",
        "golangci_lint",
        "golangci",
        "clang_tidy",
        "ClangTidy",
        "ShellCheck",
        "shell-check",
        "PyLint",
        "Git",
        "GIT",
        "git ",
        "Vulture",
        "Periphery",
        "VULTURE",
        "Gitleaks",
        "cargo_audit",
        "cargo audit",
    ] {
        assert_eq!(Engine::from_name(name), None, "{name:?}");
    }
}
