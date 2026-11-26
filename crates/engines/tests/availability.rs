use std::path::Path;

use engines::{availability_in, Availability};
use rules::Engine;

#[test]
fn installed_engines_are_available() {
    let engines = [Engine::Clippy, Engine::SwiftLint, Engine::Ruff, Engine::AstGrep];
    let results = engines.map(|engine| availability_in(engine, Path::new(env!("CARGO_MANIFEST_DIR"))));
    assert_eq!(results, [Availability::Available, Availability::Available, Availability::Available, Availability::Available]);
}
