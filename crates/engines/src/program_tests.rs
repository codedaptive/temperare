//! ZUT for program.rs: the program each engine runs as (the plan's table).

use crate::program::program;
use rules::Engine;

#[test]
fn each_engine_runs_as_its_program() {
    assert_eq!(program(Engine::Clippy), "cargo");
    assert_eq!(program(Engine::SwiftLint), "swiftlint");
    assert_eq!(program(Engine::Ruff), "ruff");
    assert_eq!(program(Engine::AstGrep), "ast-grep");
    assert_eq!(program(Engine::Tessera), "ts");
}
