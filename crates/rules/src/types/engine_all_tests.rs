//! ZUT for Engine::ALL: declaration order, completeness and name round trips.

use super::Engine;

fn position(engine: Engine) -> usize {
    match engine {
        Engine::Clippy => 0,
        Engine::SwiftLint => 1,
        Engine::Ruff => 2,
        Engine::AstGrep => 3,
        Engine::Tessera => 4,
        Engine::GolangciLint => 5,
        Engine::Pylint => 6,
        Engine::ClangTidy => 7,
        Engine::ShellCheck => 8,
        Engine::Git => 9,
        Engine::Vulture => 10,
        Engine::Periphery => 11,
        Engine::Gitleaks => 12,
        Engine::CargoAudit => 13,
        Engine::Eslint => 14,
        Engine::Mypy => 15,
        Engine::Pmd => 16,
        Engine::Detekt => 17,
        Engine::Rubocop => 18,
        Engine::Phpstan => 19,
        Engine::Dart => 20,
        Engine::Markdownlint => 21,
        Engine::Yamllint => 22,
        Engine::Actionlint => 23,
        Engine::Hadolint => 24,
        Engine::Vale => 25,
        Engine::Stylelint => 26,
        Engine::Sqlfluff => 27,
    }
}

#[test]
fn all_contains_each_variant_once_in_declaration_order() {
    assert_eq!(Engine::ALL.len(), 28);
    for (index, engine) in Engine::ALL.into_iter().enumerate() {
        assert_eq!(position(engine), index, "{engine:?} at {index}");
    }
}

#[test]
fn round_trips_through_from_name() {
    for engine in Engine::ALL {
        assert_eq!(Engine::from_name(engine.name()), Some(engine), "{engine:?}");
    }
}
