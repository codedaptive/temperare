//! Unit tests for engine_needs_graph.rs: Tessera only needs the graph.

use super::Engine;

#[test]
fn tessera_needs_the_graph() {
    assert!(Engine::Tessera.needs_graph());
}

#[test]
fn the_other_four_do_not() {
    assert!(!Engine::Clippy.needs_graph());
    assert!(!Engine::SwiftLint.needs_graph());
    assert!(!Engine::Ruff.needs_graph());
    assert!(!Engine::AstGrep.needs_graph());
}
