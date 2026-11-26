use super::list_version;
use rules::Engine;

#[test]
fn list_versions_name_the_releases_the_lists_were_taken_from() {
    let cases = [
        ("shellcheck", Engine::ShellCheck, Some("0.11.0")),
        ("hadolint", Engine::Hadolint, Some("2.15.1")),
        ("detekt", Engine::Detekt, Some("1.23.8")),
        ("markdownlint", Engine::Markdownlint, Some("0.24.2")),
        ("no list", Engine::Ruff, None),
    ];
    for (name, engine, expected) in cases {
        assert_eq!(list_version(engine), expected, "{name}");
    }
}
