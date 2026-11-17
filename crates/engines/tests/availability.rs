use engines::{availability, Availability};
use rules::Engine;

#[test]
fn installed_engines_are_available() {
    let engines = [Engine::Clippy, Engine::SwiftLint, Engine::Ruff, Engine::AstGrep];
    let results = engines.map(availability);
    assert_eq!(results, [Availability::Available, Availability::Available, Availability::Available, Availability::Available]);
}
