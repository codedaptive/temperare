//! ZUT for report_engines.rs: pure mapping, no engine runs.

use super::report_engines::report_engines;
use rules::Engine;

fn named(engine: Engine, version: Option<&str>) -> engines::EngineVersion {
    engines::EngineVersion { engine, version: version.map(str::to_string) }
}

#[test]
fn each_engine_and_version_is_carried() {
    let mapped = report_engines(vec![named(Engine::Ruff, Some("0.17.4")), named(Engine::SwiftLint, None)]);
    assert_eq!(mapped.len(), 2);
    assert_eq!(mapped[0].engine, Engine::Ruff);
    assert_eq!(mapped[0].version.as_deref(), Some("0.17.4"));
    assert_eq!(mapped[1].engine, Engine::SwiftLint);
    assert_eq!(mapped[1].version, None);
}

#[test]
fn a_version_serializes_as_a_string_and_none_as_null() {
    let mapped = report_engines(vec![named(Engine::Ruff, Some("0.17.4")), named(Engine::SwiftLint, None)]);
    assert_eq!(
        serde_json::to_string(&mapped).unwrap(),
        r#"[{"engine":"ruff","version":"0.17.4"},{"engine":"swiftlint","version":null}]"#
    );
}

#[test]
fn the_order_is_kept() {
    let mapped = report_engines(vec![
        named(Engine::ShellCheck, Some("0.10.0")),
        named(Engine::AstGrep, None),
        named(Engine::Clippy, Some("0.1.83")),
    ]);
    assert_eq!(
        mapped,
        vec![
            report::EngineVersion { engine: Engine::ShellCheck, version: Some("0.10.0".to_string()) },
            report::EngineVersion { engine: Engine::AstGrep, version: None },
            report::EngineVersion { engine: Engine::Clippy, version: Some("0.1.83".to_string()) },
        ]
    );
}

#[test]
fn no_engines_give_no_entries() {
    assert_eq!(report_engines(vec![]), vec![]);
}
