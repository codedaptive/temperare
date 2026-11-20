//! ZUT for engine_name.rs: each engine's rule-file name, the round trip through `Engine::from_name`, and agreement
//! with the serde name.

use super::Engine;
use serde::Serialize;

const ALL: [Engine; 5] = [Engine::Clippy, Engine::SwiftLint, Engine::Ruff, Engine::AstGrep, Engine::Tessera];

#[test]
fn clippy() {
    assert_eq!(Engine::Clippy.name(), "clippy");
}

#[test]
fn swiftlint() {
    assert_eq!(Engine::SwiftLint.name(), "swiftlint");
}

#[test]
fn ruff() {
    assert_eq!(Engine::Ruff.name(), "ruff");
}

#[test]
fn ast_grep() {
    assert_eq!(Engine::AstGrep.name(), "ast-grep");
}

#[test]
fn tessera() {
    assert_eq!(Engine::Tessera.name(), "tessera");
}

#[test]
fn round_trips_through_from_name() {
    for engine in ALL {
        assert_eq!(Engine::from_name(engine.name()), Some(engine), "{engine:?}");
    }
}

#[test]
fn equals_the_serde_name() {
    #[derive(Serialize)]
    struct Wrap {
        engine: Engine,
    }
    for engine in ALL {
        let written = toml::to_string(&Wrap { engine }).unwrap();
        assert_eq!(written, format!("engine = \"{}\"\n", engine.name()), "{engine:?}");
    }
}
