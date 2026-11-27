//! Unit test for entry_of.rs: every engine has its own entry and rule-file name, and the table covers every engine once.

use crate::entry_of::entry_of;
use rules::Engine;

const ENGINES: [(Engine, &str); 28] = [
    (Engine::Clippy, "clippy"),
    (Engine::SwiftLint, "swiftlint"),
    (Engine::Ruff, "ruff"),
    (Engine::AstGrep, "ast-grep"),
    (Engine::Tessera, "tessera"),
    (Engine::ShellCheck, "shellcheck"),
    (Engine::Git, "git"),
    (Engine::Pylint, "pylint"),
    (Engine::ClangTidy, "clang-tidy"),
    (Engine::GolangciLint, "golangci-lint"),
    (Engine::Gitleaks, "gitleaks"),
    (Engine::CargoAudit, "cargo-audit"),
    (Engine::Vulture, "vulture"),
    (Engine::Periphery, "periphery"),
    (Engine::Rubocop, "rubocop"),
    (Engine::Pmd, "pmd"),
    (Engine::Mypy, "mypy"),
    (Engine::Detekt, "detekt"),
    (Engine::Eslint, "eslint"),
    (Engine::Dart, "dart"),
    (Engine::Phpstan, "phpstan"),
    (Engine::Actionlint, "actionlint"),
    (Engine::Sqlfluff, "sqlfluff"),
    (Engine::Hadolint, "hadolint"),
    (Engine::Yamllint, "yamllint"),
    (Engine::Vale, "vale"),
    (Engine::Stylelint, "stylelint"),
    (Engine::Markdownlint, "markdownlint"),
];

#[test]
fn entry_of_maps_every_engine_to_its_named_entry() {
    for engine in Engine::ALL {
        assert_eq!(ENGINES.iter().filter(|(listed, _)| *listed == engine).count(), 1, "{engine:?}");
    }
    for (engine, name) in ENGINES {
        let entry = entry_of(engine);
        assert_eq!(entry.engine, engine, "{engine:?}");
        assert_eq!(entry.name, name, "{engine:?}");
        assert_eq!(name, engine.name(), "{engine:?}");
        assert_eq!(serde_json::to_value(engine).unwrap(), name, "{engine:?}");
    }
}
