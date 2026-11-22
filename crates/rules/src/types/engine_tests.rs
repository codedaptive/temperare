//! ZUT for engine.rs: each variant uses its rule-file name for serde.

use super::Engine;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Wrap {
    engine: Engine,
}

#[test]
fn each_engine_serializes_to_its_rule_file_name() {
    for (engine, name) in [
        (Engine::Clippy, "clippy"),
        (Engine::SwiftLint, "swiftlint"),
        (Engine::Ruff, "ruff"),
        (Engine::AstGrep, "ast-grep"),
        (Engine::Tessera, "tessera"),
        (Engine::GolangciLint, "golangci-lint"),
        (Engine::Pylint, "pylint"),
        (Engine::ClangTidy, "clang-tidy"),
        (Engine::ShellCheck, "shellcheck"),
        (Engine::Git, "git"),
        (Engine::Vulture, "vulture"),
        (Engine::Periphery, "periphery"),
        (Engine::Gitleaks, "gitleaks"),
        (Engine::CargoAudit, "cargo-audit"),
        (Engine::Eslint, "eslint"),
        (Engine::Mypy, "mypy"),
        (Engine::Pmd, "pmd"),
        (Engine::Detekt, "detekt"),
        (Engine::Rubocop, "rubocop"),
        (Engine::Phpstan, "phpstan"),
        (Engine::Dart, "dart"),
        (Engine::Markdownlint, "markdownlint"),
        (Engine::Yamllint, "yamllint"),
        (Engine::Actionlint, "actionlint"),
        (Engine::Hadolint, "hadolint"),
        (Engine::Vale, "vale"),
        (Engine::Stylelint, "stylelint"),
        (Engine::Sqlfluff, "sqlfluff"),
    ] {
        let text = format!("engine = \"{name}\"\n");
        assert_eq!(toml::to_string(&Wrap { engine }).unwrap(), text, "{engine:?}, {name:?}");
        let read: Wrap = toml::from_str(&text).unwrap();
        assert_eq!(read.engine, engine, "{engine:?}, {name:?}");
    }
}
