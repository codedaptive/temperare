use super::list_floor;
use crate::types::EngineVersion;
use rules::Engine;

#[test]
fn older_readable_versions_give_the_list_floor_reason() {
    let cases = [
        ("shellcheck older", Engine::ShellCheck, Some("0.10.0"), Some("shellcheck 0.10.0 is older than shellcheck 0.11.0, the release temperare's list of shellcheck codes was taken from")),
        ("shellcheck numeric comparison", Engine::ShellCheck, Some("0.9.0"), Some("shellcheck 0.9.0 is older than shellcheck 0.11.0, the release temperare's list of shellcheck codes was taken from")),
        ("shellcheck equal", Engine::ShellCheck, Some("0.11.0"), None),
        ("shellcheck newer", Engine::ShellCheck, Some("0.12.0"), None),
        ("hadolint older", Engine::Hadolint, Some("2.14.0"), Some("hadolint 2.14.0 is older than hadolint 2.15.1, the release temperare's list of hadolint codes was taken from")),
        ("hadolint equal", Engine::Hadolint, Some("2.15.1"), None),
        ("detekt older", Engine::Detekt, Some("1.23.7"), Some("detekt 1.23.7 is older than detekt 1.23.8, the release temperare's list of detekt codes was taken from")),
        ("detekt equal", Engine::Detekt, Some("1.23.8"), None),
        ("markdownlint older", Engine::Markdownlint, Some("0.23.0"), Some("markdownlint-cli2 0.23.0 is older than markdownlint-cli2 0.24.2, the release bundling markdownlint 0.42.2, which temperare's list of markdownlint codes was taken from")),
        ("markdownlint equal", Engine::Markdownlint, Some("0.24.2"), None),
        ("prefix unreadable", Engine::Hadolint, Some("v2.14.0"), None),
        ("two parts unreadable", Engine::Hadolint, Some("2.15"), None),
        ("suffix unreadable", Engine::ShellCheck, Some("0.10.0-rc1"), None),
        ("empty unreadable", Engine::Hadolint, Some(""), None),
        ("version unknown", Engine::ShellCheck, None, None),
        ("engine without list", Engine::Ruff, Some("0.0.1"), None),
    ];
    let cases = cases.into_iter().map(|(name, engine, version, expected)| {
        (name, engine, vec![EngineVersion { engine, version: version.map(str::to_owned) }], expected)
    }).chain([
        ("engine absent", Engine::ShellCheck, vec![EngineVersion { engine: Engine::Ruff, version: Some("0.0.1".into()) }], None),
        ("first matching version wins", Engine::ShellCheck, vec![
            EngineVersion { engine: Engine::ShellCheck, version: Some("0.11.0".into()) },
            EngineVersion { engine: Engine::ShellCheck, version: Some("0.10.0".into()) },
        ], None),
    ]);
    for (name, engine, versions, expected) in cases {
        assert_eq!(list_floor(engine, &versions).as_deref(), expected, "{name}");
    }
}
