//! Test support: an engine with no adapter in this build, for the tests of the no-adapter paths.

use rules::Engine;

/// The first engine of `Engine::ALL` whose `entry_for` is `None`. Panics when every engine has an adapter, so the
/// no-adapter tests fail rather than pass with nothing to test.
pub(crate) fn an_engine_with_no_adapter() -> Engine {
    Engine::ALL
        .into_iter()
        .find(|e| crate::entry_for::entry_for(*e).is_none())
        .expect("every engine in Engine::ALL has an adapter, so the no-adapter tests have nothing to test with")
}
