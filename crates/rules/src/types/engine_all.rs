//! `Engine::ALL`: every engine, in declaration order.

use super::Engine;

impl Engine {
    /// Every variant of `Engine`, in declaration order. The rule-file reader builds its "Use one of" hint for an
    /// unknown engine from it, so the hint lists each engine a rule file may name.
    pub const ALL: [Engine; 28] = [
        Engine::Clippy,
        Engine::SwiftLint,
        Engine::Ruff,
        Engine::AstGrep,
        Engine::Tessera,
        Engine::GolangciLint,
        Engine::Pylint,
        Engine::ClangTidy,
        Engine::ShellCheck,
        Engine::Git,
        Engine::Vulture,
        Engine::Periphery,
        Engine::Gitleaks,
        Engine::CargoAudit,
        Engine::Eslint,
        Engine::Mypy,
        Engine::Pmd,
        Engine::Detekt,
        Engine::Rubocop,
        Engine::Phpstan,
        Engine::Dart,
        Engine::Markdownlint,
        Engine::Yamllint,
        Engine::Actionlint,
        Engine::Hadolint,
        Engine::Vale,
        Engine::Stylelint,
        Engine::Sqlfluff,
    ];
}
