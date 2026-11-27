//! `Engine`: the program a rule runs.

use serde::{Deserialize, Serialize};

/// An engine Tao runs as a program. Each variant is renamed by hand to its rule-file name: `rename_all` would
/// give "ast_grep" and "swift_lint".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Engine {
    #[serde(rename = "clippy")]
    Clippy,
    #[serde(rename = "swiftlint")]
    SwiftLint,
    #[serde(rename = "ruff")]
    Ruff,
    #[serde(rename = "ast-grep")]
    AstGrep,
    #[serde(rename = "tessera")]
    Tessera,
    #[serde(rename = "golangci-lint")]
    GolangciLint,
    #[serde(rename = "pylint")]
    Pylint,
    #[serde(rename = "clang-tidy")]
    ClangTidy,
    #[serde(rename = "shellcheck")]
    ShellCheck,
    #[serde(rename = "git")]
    Git,
    #[serde(rename = "vulture")]
    Vulture,
    #[serde(rename = "periphery")]
    Periphery,
    #[serde(rename = "gitleaks")]
    Gitleaks,
    #[serde(rename = "cargo-audit")]
    CargoAudit,
    #[serde(rename = "eslint")]
    Eslint,
    #[serde(rename = "mypy")]
    Mypy,
    #[serde(rename = "pmd")]
    Pmd,
    #[serde(rename = "detekt")]
    Detekt,
    #[serde(rename = "rubocop")]
    Rubocop,
    #[serde(rename = "phpstan")]
    Phpstan,
    #[serde(rename = "dart")]
    Dart,
    #[serde(rename = "markdownlint")]
    Markdownlint,
    #[serde(rename = "yamllint")]
    Yamllint,
    #[serde(rename = "actionlint")]
    Actionlint,
    #[serde(rename = "hadolint")]
    Hadolint,
    #[serde(rename = "vale")]
    Vale,
    #[serde(rename = "stylelint")]
    Stylelint,
    #[serde(rename = "sqlfluff")]
    Sqlfluff,
}
