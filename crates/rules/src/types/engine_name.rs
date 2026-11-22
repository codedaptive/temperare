//! `Engine::name`: an engine's rule-file name.

use super::Engine;

impl Engine {
    /// The engine's rule-file name, the one `Engine::from_name` reads and serde writes.
    pub fn name(self) -> &'static str {
        match self {
            Engine::Clippy => "clippy",
            Engine::SwiftLint => "swiftlint",
            Engine::Ruff => "ruff",
            Engine::AstGrep => "ast-grep",
            Engine::Tessera => "tessera",
            Engine::GolangciLint => "golangci-lint",
            Engine::Pylint => "pylint",
            Engine::ClangTidy => "clang-tidy",
            Engine::Oclint => "oclint",
            Engine::ShellCheck => "shellcheck",
            Engine::Git => "git",
            Engine::Vulture => "vulture",
            Engine::Periphery => "periphery",
            Engine::Gitleaks => "gitleaks",
            Engine::CargoAudit => "cargo-audit",
        }
    }
}
