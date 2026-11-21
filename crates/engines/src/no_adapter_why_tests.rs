//! Unit tests for no_adapter_why.rs: the why names the engine by its rule-file name.

use crate::no_adapter_why::no_adapter_why;
use rules::Engine;

#[test]
fn the_why_names_each_engine_by_its_rule_file_name() {
    assert_eq!(no_adapter_why(Engine::Clippy), "no adapter for clippy in this build");
    assert_eq!(no_adapter_why(Engine::SwiftLint), "no adapter for swiftlint in this build");
    assert_eq!(no_adapter_why(Engine::Ruff), "no adapter for ruff in this build");
    assert_eq!(no_adapter_why(Engine::AstGrep), "no adapter for ast-grep in this build");
    assert_eq!(no_adapter_why(Engine::Tessera), "no adapter for tessera in this build");
}
