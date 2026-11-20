//! ZUT for engine_needs_graph.rs: Tessera only needs the graph.

use super::Engine;

#[test]
fn tessera_needs_the_graph() {
    assert!(Engine::Tessera.needs_graph());
}

#[test]
fn the_others_do_not() {
    assert!(!Engine::Clippy.needs_graph());
    assert!(!Engine::SwiftLint.needs_graph());
    assert!(!Engine::Ruff.needs_graph());
    assert!(!Engine::AstGrep.needs_graph());
    assert!(!Engine::GolangciLint.needs_graph());
    assert!(!Engine::Pylint.needs_graph());
    assert!(!Engine::ClangTidy.needs_graph());
    assert!(!Engine::Oclint.needs_graph());
    assert!(!Engine::ShellCheck.needs_graph());
}
