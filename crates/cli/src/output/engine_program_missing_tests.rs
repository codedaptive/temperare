//! ZUT for engine_program_missing.rs: which whys name one of the engine's own programs.

use super::engine_program_missing::engine_program_missing;
use rules::Engine;

#[test]
fn engine_program_missing_matches_only_the_engines_needs() {
    let rows = [
        (Engine::Clippy, "cargo-clippy not found on PATH", true),
        (Engine::Clippy, "clippy-driver not found on PATH", true),
        (Engine::Clippy, "cargo not found on PATH", false),
        (Engine::Ruff, "ruff not found on PATH", true),
        (Engine::Ruff, "xruff not found on PATH", false),
        (Engine::ShellCheck, "git not found on PATH", false),
        (Engine::Ruff, "ruff 0.4.1 does not know the check `X`: y", false),
    ];
    let wrong: Vec<String> = rows
        .into_iter()
        .filter(|&(engine, why, want)| engine_program_missing(engine, why) != want)
        .map(|(engine, why, want)| format!("{engine:?} {why}: want {want}"))
        .collect();
    assert!(wrong.is_empty(), "rows wrong:\n{}", wrong.join("\n"));
}
