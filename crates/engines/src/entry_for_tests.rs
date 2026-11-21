//! Unit tests for entry_for.rs: each engine maps to the entry serving it, named by the engine's rule-file name.

use crate::entry_for::entry_for;
use rules::Engine;

const ENGINES: [(Engine, &str); 5] = [
    (Engine::Clippy, "clippy"),
    (Engine::SwiftLint, "swiftlint"),
    (Engine::Ruff, "ruff"),
    (Engine::AstGrep, "ast-grep"),
    (Engine::Tessera, "tessera"),
];

#[test]
fn each_engine_maps_to_the_entry_serving_it() {
    for (engine, _) in ENGINES {
        assert_eq!(entry_for(engine).unwrap().engine, engine, "{engine:?}");
    }
}

#[test]
fn each_entry_is_named_by_its_rule_file_name() {
    for (engine, name) in ENGINES {
        assert_eq!(entry_for(engine).unwrap().name, name, "{engine:?}");
        assert_eq!(serde_json::to_value(engine).unwrap(), name, "{engine:?}");
    }
}

#[test]
fn each_entry_name_is_the_engine_name() {
    for (engine, _) in ENGINES {
        assert_eq!(entry_for(engine).unwrap().name, engine.name(), "{engine:?}");
    }
}
