//! `entry_for`: the registry of engines, each engine's entry.

use crate::ast_grep::AST_GREP;
use crate::clippy::CLIPPY;
use crate::entry::Entry;
use crate::ruff::RUFF;
use crate::swiftlint::SWIFTLINT;
use crate::tessera::TESSERA;
use rules::Engine;

/// The entry of `engine`. The match is exhaustive, one line per engine: an adapter for a new engine is its entry
/// `static` in its own folder plus its one line here.
pub fn entry_for(engine: Engine) -> &'static Entry {
    match engine {
        Engine::Clippy => &CLIPPY,
        Engine::SwiftLint => &SWIFTLINT,
        Engine::Ruff => &RUFF,
        Engine::AstGrep => &AST_GREP,
        Engine::Tessera => &TESSERA,
    }
}
