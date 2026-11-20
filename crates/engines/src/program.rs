//! `program`: the program each engine runs as.

use rules::Engine;

/// The program name an engine is run as: clippy runs through cargo, Tessera as `ts`. `tessera_cmd` names `ts`, but
/// no rule runs it or looks it up while Tessera's slop records are not defined.
pub(crate) fn program(engine: Engine) -> &'static str {
    match engine {
        Engine::Clippy => "cargo",
        Engine::SwiftLint => "swiftlint",
        Engine::Ruff => "ruff",
        Engine::AstGrep => "ast-grep",
        Engine::Tessera => "ts",
    }
}
