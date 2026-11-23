//! `entry_for`: the registry of engines, each engine's entry.

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

/// The entry of `engine`: `Some` for every engine, since each has an adapter. The `Option` is kept for its callers,
/// which handle `None` as not available (settled S-73). An adapter for a new engine is its entry `static` in its own
/// folder plus its one arm here.
pub fn entry_for(engine: Engine) -> Option<&'static Entry> {
    match engine {
        Engine::Clippy => Some(&CLIPPY),
        Engine::SwiftLint => Some(&SWIFTLINT),
        Engine::Ruff => Some(&RUFF),
        Engine::AstGrep => Some(&AST_GREP),
        Engine::Tessera => Some(&TESSERA),
        Engine::ShellCheck => Some(&SHELLCHECK),
        Engine::Git => Some(&GIT),
        Engine::Pylint => Some(&PYLINT),
        Engine::ClangTidy => Some(&CLANG_TIDY),
        Engine::GolangciLint => Some(&GOLANGCI_LINT),
        Engine::Gitleaks => Some(&GITLEAKS),
        Engine::CargoAudit => Some(&CARGO_AUDIT),
        Engine::Vulture => Some(&VULTURE),
        Engine::Periphery => Some(&PERIPHERY),
        Engine::Rubocop => Some(&RUBOCOP),
        Engine::Pmd => Some(&PMD),
        Engine::Mypy => Some(&MYPY),
        Engine::Detekt => Some(&DETEKT),
        Engine::Eslint => Some(&ESLINT),
        Engine::Dart => Some(&DART),
        Engine::Phpstan => Some(&PHPSTAN),
        Engine::Actionlint => Some(&ACTIONLINT),
        Engine::Sqlfluff => Some(&SQLFLUFF),
        Engine::Hadolint => Some(&HADOLINT),
        Engine::Yamllint => Some(&YAMLLINT),
        Engine::Vale => Some(&VALE),
        Engine::Stylelint => Some(&STYLELINT),
        Engine::Markdownlint => Some(&MARKDOWNLINT),
    }
}
