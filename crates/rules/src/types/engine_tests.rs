//! ZUT for engine.rs: each variant serializes to its rule-file name (SPEC-rules: the engine names).

use super::Engine;
use serde::Serialize;

#[derive(Serialize)]
struct Wrap {
    engine: Engine,
}

fn serialized(engine: Engine) -> String {
    toml::to_string(&Wrap { engine }).unwrap()
}

#[test]
fn each_engine_serializes_to_its_rule_file_name() {
    assert_eq!(serialized(Engine::Clippy), "engine = \"clippy\"\n");
    assert_eq!(serialized(Engine::SwiftLint), "engine = \"swiftlint\"\n");
    assert_eq!(serialized(Engine::Ruff), "engine = \"ruff\"\n");
    assert_eq!(serialized(Engine::AstGrep), "engine = \"ast-grep\"\n");
    assert_eq!(serialized(Engine::Tessera), "engine = \"tessera\"\n");
    assert_eq!(serialized(Engine::GolangciLint), "engine = \"golangci-lint\"\n");
    assert_eq!(serialized(Engine::Pylint), "engine = \"pylint\"\n");
    assert_eq!(serialized(Engine::ClangTidy), "engine = \"clang-tidy\"\n");
    assert_eq!(serialized(Engine::Oclint), "engine = \"oclint\"\n");
    assert_eq!(serialized(Engine::ShellCheck), "engine = \"shellcheck\"\n");
    assert_eq!(serialized(Engine::Git), "engine = \"git\"\n");
    assert_eq!(serialized(Engine::Vulture), "engine = \"vulture\"\n");
    assert_eq!(serialized(Engine::Periphery), "engine = \"periphery\"\n");
    assert_eq!(serialized(Engine::Gitleaks), "engine = \"gitleaks\"\n");
    assert_eq!(serialized(Engine::CargoAudit), "engine = \"cargo-audit\"\n");
}

#[test]
fn engine_reads_back_from_its_rule_file_name() {
    #[derive(serde::Deserialize)]
    struct Read {
        engine: Engine,
    }
    let read: Read = toml::from_str("engine = \"ast-grep\"").unwrap();
    assert_eq!(read.engine, Engine::AstGrep);
}
