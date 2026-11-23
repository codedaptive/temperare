//! Unit tests for entry_for.rs: each engine with an adapter maps to the entry serving it, named by the engine's rule-file
//! name, and every other engine has no entry.

use crate::entry_for::entry_for;
use rules::Engine;

const ENGINES: [(Engine, &str); 14] = [
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
];

#[test]
fn each_engine_maps_to_the_entry_serving_it() {
    for (engine, _) in ENGINES {
        assert_eq!(entry_for(engine).unwrap().engine, engine, "{engine:?}");
    }
}

#[test]
fn every_engine_outside_the_table_has_no_entry() {
    for engine in Engine::ALL {
        let in_table = ENGINES.iter().any(|(listed, _)| *listed == engine);
        assert_eq!(entry_for(engine).is_some(), in_table, "{engine:?}");
    }
}

#[test]
fn each_entry_is_named_by_its_rule_file_name() {
    for (engine, name) in ENGINES {
        assert_eq!(entry_for(engine).unwrap().name, name, "{engine:?}");
        assert_eq!(serde_json::to_value(engine).unwrap(), name, "{engine:?}");
    }
}

#[test]
fn each_entry_name_is_the_engine_name() {
    for (engine, _) in ENGINES {
        assert_eq!(entry_for(engine).unwrap().name, engine.name(), "{engine:?}");
    }
}

#[test]
fn shellcheck_maps_to_the_shellcheck_entry() {
    let entry = entry_for(Engine::ShellCheck).unwrap();
    assert_eq!((entry.name, entry.engine), ("shellcheck", Engine::ShellCheck));
}

#[test]
fn git_maps_to_the_git_entry() {
    let entry = entry_for(Engine::Git).unwrap();
    assert_eq!((entry.name, entry.engine), ("git", Engine::Git));
}

