//! Unit test for hadolint_entry.rs: the fields of `HADOLINT` a rule file relies on, and its version probe read on hadolint
//! 2.15.1's `--version` output. Nothing runs.

use super::HADOLINT;
use crate::entry::Pick;
use procio::Output;
use rules::Engine;

#[test]
fn hadolint_entry_fields() {
    assert_eq!((HADOLINT.name, HADOLINT.engine, HADOLINT.program), ("hadolint", Engine::Hadolint, "hadolint"));
    assert_eq!(HADOLINT.needs, ["hadolint"]);
    assert_eq!(HADOLINT.args, ["--no-color", "-f", "sarif", "--disable-ignore-pragma"]);
    assert_eq!(HADOLINT.pick, Pick::Arg("--error="));

    let probe = HADOLINT.version.unwrap();
    assert_eq!(probe.args, ["--version"]);
    let version = Output { code: Some(0), stdout: b"Haskell Dockerfile Linter 2.15.1\n".to_vec(), stderr: Vec::new() };
    assert_eq!((probe.read)(&version), Some("2.15.1".to_string()));
}
