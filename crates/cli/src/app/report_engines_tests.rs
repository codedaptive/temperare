//! ZUT for report_engines.rs: pure filtering and mapping, no engine runs and no PATH lookup (the availability is a
//! closure; the version probe is the engines crate's static entry).

use super::report_engines::report_engines;
use engines::Availability;
use rules::Engine;

fn named(engine: Engine, version: Option<&str>) -> engines::EngineVersion {
    engines::EngineVersion { engine, version: version.map(str::to_string) }
}

fn entry(engine: Engine, version: Option<&str>) -> report::EngineVersion {
    report::EngineVersion { engine, version: version.map(str::to_string) }
}

fn found(_: Engine) -> Availability {
    Availability::Available
}

#[test]
fn an_engine_not_found_here_is_left_out() {
    let absent_swiftlint = |engine: Engine| match engine {
        Engine::SwiftLint => Availability::NotAvailable { why: "swiftlint not found on PATH".to_string() },
        _ => Availability::Available,
    };
    let mapped =
        report_engines(vec![named(Engine::SwiftLint, None), named(Engine::Ruff, Some("0.17.4"))], &absent_swiftlint);
    assert_eq!(mapped, vec![entry(Engine::Ruff, Some("0.17.4"))]);
}

#[test]
fn a_found_engine_with_no_version_read_is_kept_with_none() {
    let mapped = report_engines(vec![named(Engine::Ruff, Some("0.17.4")), named(Engine::SwiftLint, None)], &found);
    assert_eq!(mapped, vec![entry(Engine::Ruff, Some("0.17.4")), entry(Engine::SwiftLint, None)]);
}

#[test]
fn a_version_serializes_as_a_string_and_none_as_null() {
    let mapped = report_engines(vec![named(Engine::Ruff, Some("0.17.4")), named(Engine::SwiftLint, None)], &found);
    assert_eq!(
        serde_json::to_string(&mapped).unwrap(),
        r#"[{"engine":"ruff","version":"0.17.4"},{"engine":"swiftlint","version":null}]"#
    );
}

#[test]
fn tessera_is_left_out_even_when_found_as_it_has_no_version_probe() {
    let mapped = report_engines(vec![named(Engine::Tessera, None), named(Engine::Clippy, Some("0.1.83"))], &found);
    assert_eq!(mapped, vec![entry(Engine::Clippy, Some("0.1.83"))]);
}

#[test]
fn an_engine_with_no_adapter_is_left_out_even_when_found() {
    let mapped = report_engines(vec![named(Engine::ShellCheck, Some("0.10.0")), named(Engine::Ruff, None)], &found);
    assert_eq!(mapped, vec![entry(Engine::Ruff, None)]);
}

#[test]
fn the_first_use_order_is_kept() {
    // SwiftLint, AstGrep, Ruff, Clippy matches no sort a wrong implementation could fall back on: by serialized name
    // it would be ast-grep, clippy, ruff, swiftlint; reversed, swiftlint, ruff, clippy, ast-grep; by the enum's
    // declaration order Clippy comes first, and by version text the versionless AstGrep would lead or trail.
    let mapped = report_engines(
        vec![
            named(Engine::SwiftLint, Some("0.57.0")),
            named(Engine::AstGrep, None),
            named(Engine::Ruff, Some("0.17.4")),
            named(Engine::Clippy, Some("0.1.83")),
        ],
        &found,
    );
    assert_eq!(
        mapped,
        vec![
            entry(Engine::SwiftLint, Some("0.57.0")),
            entry(Engine::AstGrep, None),
            entry(Engine::Ruff, Some("0.17.4")),
            entry(Engine::Clippy, Some("0.1.83")),
        ]
    );
}

#[test]
fn no_engines_give_no_entries() {
    assert_eq!(report_engines(vec![], &found), vec![]);
}
