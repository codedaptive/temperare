//! `Engine`: the program a rule runs.

use serde::{Deserialize, Serialize};

/// An engine Temperare runs as a program. Each variant is renamed by hand to its rule-file name: `rename_all` would
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
}
