//! ZUT for engine_from_name.rs: rule-file names, and None for anything else.

use super::Engine;

#[test]
fn each_rule_file_name_is_read() {
    for (name, engine) in [
        ("clippy", Engine::Clippy),
        ("swiftlint", Engine::SwiftLint),
        ("ruff", Engine::Ruff),
        ("ast-grep", Engine::AstGrep),
        ("tessera", Engine::Tessera),
        ("golangci-lint", Engine::GolangciLint),
        ("pylint", Engine::Pylint),
        ("clang-tidy", Engine::ClangTidy),
        ("shellcheck", Engine::ShellCheck),
        ("git", Engine::Git),
        ("vulture", Engine::Vulture),
        ("periphery", Engine::Periphery),
        ("gitleaks", Engine::Gitleaks),
        ("cargo-audit", Engine::CargoAudit),
        ("eslint", Engine::Eslint),
        ("mypy", Engine::Mypy),
        ("pmd", Engine::Pmd),
        ("detekt", Engine::Detekt),
        ("rubocop", Engine::Rubocop),
        ("phpstan", Engine::Phpstan),
        ("dart", Engine::Dart),
        ("markdownlint", Engine::Markdownlint),
        ("yamllint", Engine::Yamllint),
        ("actionlint", Engine::Actionlint),
        ("hadolint", Engine::Hadolint),
        ("vale", Engine::Vale),
        ("stylelint", Engine::Stylelint),
        ("sqlfluff", Engine::Sqlfluff),
    ] {
        assert_eq!(Engine::from_name(name), Some(engine), "{name:?}, {engine:?}");
    }
}

#[test]
fn unknown_names_are_none() {
    for name in [
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
        "ESLint",
        "markdownlint-cli2",
        "sql-fluff",
        "MyPy",
        "php-stan",
        "Dart",
    ] {
        assert_eq!(Engine::from_name(name), None, "{name:?}");
    }
}
