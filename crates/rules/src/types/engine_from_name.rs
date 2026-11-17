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
            _ => None,
        }
    }
}
