//! `Engine::from_name`: an engine from its rule-file name.

use super::Engine;

impl Engine {
    /// The engine named `name` in a rule file, or None for any other text (names are exact and lower-case).
    pub fn from_name(name: &str) -> Option<Engine> {
        match name {
            "clippy" => Some(Engine::Clippy),
            "swiftlint" => Some(Engine::SwiftLint),
            "ruff" => Some(Engine::Ruff),
            "ast-grep" => Some(Engine::AstGrep),
            "tessera" => Some(Engine::Tessera),
            "golangci-lint" => Some(Engine::GolangciLint),
            "pylint" => Some(Engine::Pylint),
            "clang-tidy" => Some(Engine::ClangTidy),
            "shellcheck" => Some(Engine::ShellCheck),
            "git" => Some(Engine::Git),
            "vulture" => Some(Engine::Vulture),
            "periphery" => Some(Engine::Periphery),
            "gitleaks" => Some(Engine::Gitleaks),
            "cargo-audit" => Some(Engine::CargoAudit),
            "eslint" => Some(Engine::Eslint),
            "mypy" => Some(Engine::Mypy),
            "pmd" => Some(Engine::Pmd),
            "detekt" => Some(Engine::Detekt),
            "rubocop" => Some(Engine::Rubocop),
            "phpstan" => Some(Engine::Phpstan),
            "dart" => Some(Engine::Dart),
            "markdownlint" => Some(Engine::Markdownlint),
            "yamllint" => Some(Engine::Yamllint),
            "actionlint" => Some(Engine::Actionlint),
            "hadolint" => Some(Engine::Hadolint),
            "vale" => Some(Engine::Vale),
            "stylelint" => Some(Engine::Stylelint),
            "sqlfluff" => Some(Engine::Sqlfluff),
            _ => None,
        }
    }
}
