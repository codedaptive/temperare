//! `Engine::needs_graph`: whether the engine needs Tessera's code graph.

use super::Engine;

impl Engine {
    /// True for Tessera only: a rule on it is "not available" where there is no Tessera.
    pub fn needs_graph(self) -> bool {
        matches!(self, Engine::Tessera)
    }
}
