//! Unit tests for engine_needs_graph.rs: Tessera only needs the graph.

use super::Engine;

#[test]
fn tessera_needs_the_graph() {
    assert!(Engine::Tessera.needs_graph());
}

#[test]
fn the_others_do_not() {
    for engine in Engine::ALL {
        if engine != Engine::Tessera {
            assert!(!engine.needs_graph(), "{engine:?}");
        }
    }
}
