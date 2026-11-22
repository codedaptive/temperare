//! `entry_for`: the registry of engines, each engine's entry.

use crate::ast_grep::AST_GREP;
use crate::cargo_audit::CARGO_AUDIT;
use crate::clang_tidy::CLANG_TIDY;
use crate::clippy::CLIPPY;
use crate::entry::Entry;
use crate::git::GIT;
use crate::gitleaks::GITLEAKS;
use crate::golangci_lint::GOLANGCI_LINT;
use crate::pylint::PYLINT;
use crate::ruff::RUFF;
use crate::shellcheck::SHELLCHECK;
use crate::swiftlint::SWIFTLINT;
use crate::tessera::TESSERA;
use rules::Engine;

/// The entry of `engine`, `None` for an engine with no adapter in this build. An adapter for a new engine is its
/// entry `static` in its own folder plus its one arm here.
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
        // Each variant with no adapter in this build gives None, which callers report as not available with
        // `no_adapter_why`. An adapter adds its arm above.
        _ => None,
    }
}
