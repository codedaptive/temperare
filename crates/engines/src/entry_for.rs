//! `entry_for`: the registry of engines, each engine's entry.

use crate::ast_grep::AST_GREP;
use crate::clippy::CLIPPY;
use crate::entry::Entry;
use crate::ruff::RUFF;
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
        // An Engine variant with no adapter in this build gives None, which callers report as not available with
        // `no_adapter_why`. Every variant has an arm today, so this arm is unreachable; the allow silences that warning
        // until a variant without an arm exists, and the last adapter to land removes the arm and the allow.
        #[allow(unreachable_patterns)]
        _ => None,
    }
}
