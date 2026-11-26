//! ZUT for engine_version.rs: plain data; equality covers both fields, and an unknown version is not a read one.

use super::EngineVersion;
use rules::Engine;

fn ruff(version: Option<&str>) -> EngineVersion {
    EngineVersion { engine: Engine::Ruff, version: version.map(str::to_string) }
}

#[test]
fn engine_versions_compare_by_every_field() {
    assert_eq!(ruff(Some("0.17.4")), ruff(Some("0.17.4")).clone());
    assert_ne!(ruff(Some("0.17.4")), ruff(Some("0.17.5")));
    assert_ne!(ruff(Some("0.17.4")), ruff(None));
    assert_ne!(ruff(None), EngineVersion { engine: Engine::Clippy, version: None });
}
