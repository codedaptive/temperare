//! Zone integration tests for report_engines.rs, in cli's integration group, with engines' real entries: report_engines keeps an engine
//! only when `engines::entry_for` gives it an entry with a version probe, so these cases hold only while the entries
//! of ruff, SwiftLint, Clippy and ast-grep have a probe and Tessera's has none (Tessera has an entry; it is left out
//! for having no probe). The no-adapter case needs at least one variant that `entry_for` gives no entry, picked from
//! `Engine::ALL` by `entry_for`. The availability is a closure; no engine runs and no PATH lookup.

use super::super::report_engines::report_engines;
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

// Picked from entry_for rather than named, so a new adapter for any one engine leaves this case something to test.
fn an_engine_with_no_adapter() -> Engine {
    Engine::ALL
        .into_iter()
        .find(|e| engines::entry_for(*e).is_none())
        .expect("every engine has an adapter in this build, so the no-adapter case has nothing to test")
}

#[test]
fn an_engine_with_no_adapter_is_left_out_even_when_found() {
    let mapped =
        report_engines(vec![named(an_engine_with_no_adapter(), Some("3.3.1")), named(Engine::Ruff, None)], &found);
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
