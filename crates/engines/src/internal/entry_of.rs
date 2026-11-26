//! `entry_of`: the registry of engines, each engine's entry.

use crate::actionlint::ACTIONLINT;
use crate::ast_grep::AST_GREP;
use crate::cargo_audit::CARGO_AUDIT;
use crate::clang_tidy::CLANG_TIDY;
use crate::clippy::CLIPPY;
use crate::dart::DART;
use crate::detekt::DETEKT;
use crate::entry::Entry;
use crate::eslint::ESLINT;
use crate::git::GIT;
use crate::gitleaks::GITLEAKS;
use crate::golangci_lint::GOLANGCI_LINT;
use crate::hadolint::HADOLINT;
use crate::markdownlint_cli2::MARKDOWNLINT;
use crate::mypy::MYPY;
use crate::periphery::PERIPHERY;
use crate::phpstan::PHPSTAN;
use crate::pmd::PMD;
use crate::pylint::PYLINT;
use crate::rubocop::RUBOCOP;
use crate::ruff::RUFF;
use crate::shellcheck::SHELLCHECK;
use crate::sqlfluff::SQLFLUFF;
use crate::stylelint::STYLELINT;
use crate::swiftlint::SWIFTLINT;
use crate::tessera::TESSERA;
use crate::vale::VALE;
use crate::vulture::VULTURE;
use crate::yamllint::YAMLLINT;
use rules::Engine;

/// The entry of `engine`. Total: the match has one arm per `Engine`, so every engine has an entry. An adapter for a
/// new engine is its entry `static` in its own folder plus its one arm here.
pub fn entry_of(engine: Engine) -> &'static Entry {
    match engine {
        Engine::Clippy => &CLIPPY,
        Engine::SwiftLint => &SWIFTLINT,
        Engine::Ruff => &RUFF,
        Engine::AstGrep => &AST_GREP,
        Engine::Tessera => &TESSERA,
        Engine::ShellCheck => &SHELLCHECK,
        Engine::Git => &GIT,
        Engine::Pylint => &PYLINT,
        Engine::ClangTidy => &CLANG_TIDY,
        Engine::GolangciLint => &GOLANGCI_LINT,
        Engine::Gitleaks => &GITLEAKS,
        Engine::CargoAudit => &CARGO_AUDIT,
        Engine::Vulture => &VULTURE,
        Engine::Periphery => &PERIPHERY,
        Engine::Rubocop => &RUBOCOP,
        Engine::Pmd => &PMD,
        Engine::Mypy => &MYPY,
        Engine::Detekt => &DETEKT,
        Engine::Eslint => &ESLINT,
        Engine::Dart => &DART,
        Engine::Phpstan => &PHPSTAN,
        Engine::Actionlint => &ACTIONLINT,
        Engine::Sqlfluff => &SQLFLUFF,
        Engine::Hadolint => &HADOLINT,
        Engine::Yamllint => &YAMLLINT,
        Engine::Vale => &VALE,
        Engine::Stylelint => &STYLELINT,
        Engine::Markdownlint => &MARKDOWNLINT,
    }
}
