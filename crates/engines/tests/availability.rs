use std::path::Path;

use engines::{availability_in, Availability};
use rules::Engine;

#[test]
fn installed_engines_are_available() {
    let engines = [Engine::Clippy, Engine::SwiftLint, Engine::Ruff, Engine::AstGrep];
    // SwiftLint publishes no Windows build.
    for engine in engines.into_iter().filter(|engine| !cfg!(windows) || *engine != Engine::SwiftLint) {
        assert_eq!(availability_in(engine, Path::new(env!("CARGO_MANIFEST_DIR"))), Availability::Available, "{engine:?}");
    }
}
